package middleware

import (
	"github.com/gofiber/fiber/v2"
	"github.com/nicksnyder/go-i18n/v2/i18n"
	"github.com/reovo/rikkahub/server/internal/services"
)

func I18n() fiber.Handler {
	return func(c *fiber.Ctx) error {
		// 优先从 query param 获取，其次从 Accept-Language header
		lang := c.Query("lang")
		if lang == "" {
			lang = c.Get("Accept-Language", "en")
		}

		localizer := i18n.NewLocalizer(services.GetBundle(), lang)
		c.Locals("localizer", localizer)

		return c.Next()
	}
}
