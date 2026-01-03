package handlers

import (
	"errors"

	"github.com/gofiber/fiber/v2"
	"github.com/reovo/rikkahub/server/internal/middleware"
	"github.com/reovo/rikkahub/server/internal/services"
)

type AuthHandler struct {
	authService *services.AuthService
}

func NewAuthHandler(authService *services.AuthService) *AuthHandler {
	return &AuthHandler{authService: authService}
}

// Register 用户注册
// POST /api/auth/register
func (h *AuthHandler) Register(c *fiber.Ctx) error {
	var input services.RegisterInput
	if err := c.BodyParser(&input); err != nil {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": "invalid request body",
		})
	}

	// 简单验证
	if input.Username == "" || input.Email == "" || input.Password == "" {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": "username, email and password are required",
		})
	}

	if len(input.Password) < 6 {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": "password must be at least 6 characters",
		})
	}

	resp, err := h.authService.Register(&input)
	if err != nil {
		if errors.Is(err, services.ErrUserExists) {
			return c.Status(fiber.StatusConflict).JSON(fiber.Map{
				"error": "username or email already exists",
			})
		}
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": "failed to register user",
		})
	}

	return c.Status(fiber.StatusCreated).JSON(resp)
}

// Login 用户登录
// POST /api/auth/login
func (h *AuthHandler) Login(c *fiber.Ctx) error {
	var input services.LoginInput
	if err := c.BodyParser(&input); err != nil {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": "invalid request body",
		})
	}

	if input.Login == "" || input.Password == "" {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": "login and password are required",
		})
	}

	resp, err := h.authService.Login(&input)
	if err != nil {
		if errors.Is(err, services.ErrInvalidCredentials) {
			return c.Status(fiber.StatusUnauthorized).JSON(fiber.Map{
				"error": "invalid credentials",
			})
		}
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": "failed to login",
		})
	}

	return c.JSON(resp)
}

// RefreshToken 刷新令牌
// POST /api/auth/refresh
func (h *AuthHandler) RefreshToken(c *fiber.Ctx) error {
	var input struct {
		RefreshToken string `json:"refresh_token"`
	}
	if err := c.BodyParser(&input); err != nil {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": "invalid request body",
		})
	}

	if input.RefreshToken == "" {
		return c.Status(fiber.StatusBadRequest).JSON(fiber.Map{
			"error": "refresh_token is required",
		})
	}

	tokenPair, err := h.authService.RefreshToken(input.RefreshToken)
	if err != nil {
		return c.Status(fiber.StatusUnauthorized).JSON(fiber.Map{
			"error": "invalid or expired refresh token",
		})
	}

	return c.JSON(tokenPair)
}

// Me 获取当前用户信息
// GET /api/auth/me
func (h *AuthHandler) Me(c *fiber.Ctx) error {
	userID, ok := middleware.GetUserID(c)
	if !ok {
		return c.Status(fiber.StatusUnauthorized).JSON(fiber.Map{
			"error": "unauthorized",
		})
	}

	user, err := h.authService.GetUserByID(userID)
	if err != nil {
		if errors.Is(err, services.ErrUserNotFound) {
			return c.Status(fiber.StatusNotFound).JSON(fiber.Map{
				"error": "user not found",
			})
		}
		return c.Status(fiber.StatusInternalServerError).JSON(fiber.Map{
			"error": "failed to get user",
		})
	}

	return c.JSON(user)
}

// RegisterRoutes 注册认证路由
func (h *AuthHandler) RegisterRoutes(app *fiber.App, authMiddleware *middleware.AuthMiddleware) {
	auth := app.Group("/api/auth")

	auth.Post("/register", h.Register)
	auth.Post("/login", h.Login)
	auth.Post("/refresh", h.RefreshToken)
	auth.Get("/me", authMiddleware.Required(), h.Me)
}
