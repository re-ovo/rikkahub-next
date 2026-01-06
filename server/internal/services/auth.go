package services

import (
	"errors"

	"github.com/google/uuid"
	"github.com/reovo/rikkahub/server/internal/models"
	"gorm.io/gorm"
)

var (
	ErrUserNotFound       = errors.New("user not found")
	ErrUserExists         = errors.New("user already exists")
	ErrInvalidCredentials = errors.New("invalid credentials")
	ErrInvalidPassword    = errors.New("invalid password")
)

type AuthService struct {
	db         *gorm.DB
	jwtService *JWTService
}

func NewAuthService(db *gorm.DB, jwtService *JWTService) *AuthService {
	return &AuthService{db: db, jwtService: jwtService}
}

type RegisterInput struct {
	Username string `json:"username" validate:"required,min=3,max=50"`
	Email    string `json:"email" validate:"omitempty,email"`
	Password string `json:"password" validate:"required,min=6"`
	Nickname string `json:"nickname"`
}

type LoginInput struct {
	Login    string `json:"login" validate:"required"` // username 或 email
	Password string `json:"password" validate:"required"`
}

type AuthResponse struct {
	User  *models.User `json:"user"`
	Token *TokenPair   `json:"token"`
}

// Register 用户注册
func (s *AuthService) Register(input *RegisterInput) (*AuthResponse, error) {
	// 检查用户名是否已存在
	var existingUser models.User
	query := s.db.Where("username = ?", input.Username)
	if input.Email != "" {
		query = query.Or("email = ?", input.Email)
	}
	if err := query.First(&existingUser).Error; err == nil {
		return nil, ErrUserExists
	}

	// 哈希密码
	hashedPassword, err := HashPassword(input.Password)
	if err != nil {
		return nil, err
	}

	nickname := input.Nickname
	if nickname == "" {
		nickname = input.Username
	}

	user := &models.User{
		Username:     input.Username,
		PasswordHash: &hashedPassword,
		Nickname:     nickname,
		Status:       models.UserStatusActive,
	}

	// 设置邮箱（如果提供）
	if input.Email != "" {
		user.Email = &input.Email
	}

	// 创建用户
	if err := s.db.Create(user).Error; err != nil {
		return nil, err
	}

	// 分配默认用户组
	if err := s.assignDefaultGroup(user.ID); err != nil {
		return nil, err
	}

	// 生成 token
	tokenPair, err := s.jwtService.GenerateTokenPair(user.ID)
	if err != nil {
		return nil, err
	}

	return &AuthResponse{User: user, Token: tokenPair}, nil
}

// Login 用户登录
func (s *AuthService) Login(input *LoginInput) (*AuthResponse, error) {
	var user models.User
	// 优先查找用户名，如果失败则尝试邮箱
	err := s.db.Where("username = ?", input.Login).First(&user).Error
	if err != nil {
		if errors.Is(err, gorm.ErrRecordNotFound) {
			// 尝试用邮箱查找
			if err := s.db.Where("email = ?", input.Login).First(&user).Error; err != nil {
				if errors.Is(err, gorm.ErrRecordNotFound) {
					return nil, ErrInvalidCredentials
				}
				return nil, err
			}
		} else {
			return nil, err
		}
	}

	// 检查是否有密码（OAuth 用户可能没有密码）
	if user.PasswordHash == nil {
		return nil, ErrInvalidCredentials
	}

	// 验证密码
	match, err := VerifyPassword(input.Password, *user.PasswordHash)
	if err != nil || !match {
		return nil, ErrInvalidCredentials
	}

	// 检查用户状态
	if user.Status != models.UserStatusActive {
		return nil, ErrInvalidCredentials
	}

	// 生成 token
	tokenPair, err := s.jwtService.GenerateTokenPair(user.ID)
	if err != nil {
		return nil, err
	}

	return &AuthResponse{User: &user, Token: tokenPair}, nil
}

// RefreshToken 刷新 token
func (s *AuthService) RefreshToken(refreshToken string) (*TokenPair, error) {
	return s.jwtService.RefreshAccessToken(refreshToken)
}

// GetUserByID 根据ID获取用户
func (s *AuthService) GetUserByID(userID uuid.UUID) (*models.User, error) {
	var user models.User
	if err := s.db.Preload("Groups.Permissions").First(&user, "id = ?", userID).Error; err != nil {
		if errors.Is(err, gorm.ErrRecordNotFound) {
			return nil, ErrUserNotFound
		}
		return nil, err
	}
	return &user, nil
}

// assignDefaultGroup 分配默认用户组
func (s *AuthService) assignDefaultGroup(userID uuid.UUID) error {
	var defaultGroup models.Group
	if err := s.db.Where("is_default = ?", true).First(&defaultGroup).Error; err != nil {
		if errors.Is(err, gorm.ErrRecordNotFound) {
			return nil // 没有默认组，跳过
		}
		return err
	}

	userGroup := &models.UserGroup{
		UserID:  userID,
		GroupID: defaultGroup.ID,
	}
	return s.db.Create(userGroup).Error
}
