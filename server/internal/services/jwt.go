package services

import (
	"time"

	"github.com/golang-jwt/jwt/v5"
	"github.com/google/uuid"
	"github.com/reovo/rikkahub/server/internal/config"
)

type TokenType string

const (
	AccessToken  TokenType = "access"
	RefreshToken TokenType = "refresh"
)

type JWTClaims struct {
	UserID    uuid.UUID `json:"user_id"`
	TokenType TokenType `json:"token_type"`
	jwt.RegisteredClaims
}

type TokenPair struct {
	AccessToken  string `json:"access_token"`
	RefreshToken string `json:"refresh_token"`
	ExpiresIn    int64  `json:"expires_in"` // access token 过期时间（秒）
}

type JWTService struct {
	cfg *config.JWTConfig
}

func NewJWTService(cfg *config.JWTConfig) *JWTService {
	return &JWTService{cfg: cfg}
}

// GenerateTokenPair 生成 access token 和 refresh token
func (s *JWTService) GenerateTokenPair(userID uuid.UUID) (*TokenPair, error) {
	accessToken, err := s.generateToken(userID, AccessToken, s.cfg.AccessExpiresIn)
	if err != nil {
		return nil, err
	}

	refreshToken, err := s.generateToken(userID, RefreshToken, s.cfg.RefreshExpiresIn)
	if err != nil {
		return nil, err
	}

	return &TokenPair{
		AccessToken:  accessToken,
		RefreshToken: refreshToken,
		ExpiresIn:    int64(s.cfg.AccessExpiresIn.Seconds()),
	}, nil
}

// generateToken 生成单个 token
func (s *JWTService) generateToken(userID uuid.UUID, tokenType TokenType, expiresIn time.Duration) (string, error) {
	now := time.Now()
	claims := JWTClaims{
		UserID:    userID,
		TokenType: tokenType,
		RegisteredClaims: jwt.RegisteredClaims{
			ExpiresAt: jwt.NewNumericDate(now.Add(expiresIn)),
			IssuedAt:  jwt.NewNumericDate(now),
			NotBefore: jwt.NewNumericDate(now),
			Issuer:    "rikkahub",
		},
	}

	token := jwt.NewWithClaims(jwt.SigningMethodHS256, claims)
	return token.SignedString([]byte(s.cfg.Secret))
}

// ValidateToken 验证 token 并返回 claims
func (s *JWTService) ValidateToken(tokenString string) (*JWTClaims, error) {
	token, err := jwt.ParseWithClaims(tokenString, &JWTClaims{}, func(token *jwt.Token) (interface{}, error) {
		return []byte(s.cfg.Secret), nil
	})
	if err != nil {
		return nil, err
	}

	if claims, ok := token.Claims.(*JWTClaims); ok && token.Valid {
		return claims, nil
	}

	return nil, jwt.ErrSignatureInvalid
}

// RefreshAccessToken 使用 refresh token 刷新 access token
func (s *JWTService) RefreshAccessToken(refreshTokenString string) (*TokenPair, error) {
	claims, err := s.ValidateToken(refreshTokenString)
	if err != nil {
		return nil, err
	}

	if claims.TokenType != RefreshToken {
		return nil, jwt.ErrTokenInvalidClaims
	}

	return s.GenerateTokenPair(claims.UserID)
}
