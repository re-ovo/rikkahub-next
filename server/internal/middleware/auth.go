package middleware

import (
	"strings"

	"github.com/gofiber/fiber/v2"
	"github.com/google/uuid"
	"github.com/reovo/rikkahub/server/internal/services"
)

type AuthMiddleware struct {
	jwtService *services.JWTService
}

func NewAuthMiddleware(jwtService *services.JWTService) *AuthMiddleware {
	return &AuthMiddleware{jwtService: jwtService}
}

// Required 必须认证
func (m *AuthMiddleware) Required() fiber.Handler {
	return func(c *fiber.Ctx) error {
		token := m.extractToken(c)
		if token == "" {
			return c.Status(fiber.StatusUnauthorized).JSON(fiber.Map{
				"error": "missing authorization token",
			})
		}

		claims, err := m.jwtService.ValidateToken(token)
		if err != nil {
			return c.Status(fiber.StatusUnauthorized).JSON(fiber.Map{
				"error": "invalid or expired token",
			})
		}

		if claims.TokenType != services.AccessToken {
			return c.Status(fiber.StatusUnauthorized).JSON(fiber.Map{
				"error": "invalid token type",
			})
		}

		// 将用户ID存入上下文
		c.Locals("userID", claims.UserID)
		return c.Next()
	}
}

// Optional 可选认证（不强制）
func (m *AuthMiddleware) Optional() fiber.Handler {
	return func(c *fiber.Ctx) error {
		token := m.extractToken(c)
		if token == "" {
			return c.Next()
		}

		claims, err := m.jwtService.ValidateToken(token)
		if err == nil && claims.TokenType == services.AccessToken {
			c.Locals("userID", claims.UserID)
		}

		return c.Next()
	}
}

// extractToken 从请求中提取 token
func (m *AuthMiddleware) extractToken(c *fiber.Ctx) string {
	// 从 Authorization header 获取
	auth := c.Get("Authorization")
	if auth != "" && strings.HasPrefix(auth, "Bearer ") {
		return strings.TrimPrefix(auth, "Bearer ")
	}

	// 从 query 参数获取（用于 WebSocket 等场景）
	if token := c.Query("token"); token != "" {
		return token
	}

	return ""
}

// GetUserID 从上下文获取用户ID
func GetUserID(c *fiber.Ctx) (uuid.UUID, bool) {
	userID, ok := c.Locals("userID").(uuid.UUID)
	return userID, ok
}
