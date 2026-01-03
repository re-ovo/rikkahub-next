package logging

import (
	"log/slog"
	"time"

	"github.com/gofiber/fiber/v2"
)

// FiberMiddleware 创建 Fiber 日志中间件
func FiberMiddleware(logger *slog.Logger) fiber.Handler {
	return func(c *fiber.Ctx) error {
		start := time.Now()

		// 处理请求
		err := c.Next()

		// 记录日志
		status := c.Response().StatusCode()
		elapsed := time.Since(start)

		attrs := []any{
			slog.String("method", c.Method()),
			slog.String("path", c.Path()),
			slog.Int("status", status),
			slog.Duration("latency", elapsed),
			slog.String("ip", c.IP()),
		}

		// 根据状态码选择日志级别
		switch {
		case status >= 500:
			logger.Error("server error", attrs...)
		case status >= 400:
			logger.Warn("client error", attrs...)
		default:
			logger.Info("request", attrs...)
		}

		return err
	}
}
