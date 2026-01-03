package main

import (
	"log"
	"log/slog"

	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/fiber/v2/middleware/cors"
	"github.com/gofiber/fiber/v2/middleware/recover"
	"github.com/reovo/rikkahub/server/internal/config"
	"github.com/reovo/rikkahub/server/internal/database"
	"github.com/reovo/rikkahub/server/internal/handlers"
	"github.com/reovo/rikkahub/server/internal/middleware"
	"github.com/reovo/rikkahub/server/internal/models"
	"github.com/reovo/rikkahub/server/internal/services"
	"github.com/reovo/rikkahub/server/pkg/logging"
)

func main() {
	// 加载配置
	cfg, err := config.Load()
	if err != nil {
		log.Fatalf("Failed to load config: %v", err)
	}

	// 初始化 slog
	logging.Init(cfg.Debug)
	logger := logging.GetLogger()

	// 连接数据库
	db, err := database.New(&cfg.Database, cfg.Debug)
	if err != nil {
		logger.Error("Failed to connect database", slog.String("error", err.Error()))
		log.Fatalf("Failed to connect database: %v", err)
	}

	// 自动迁移
	if err := models.AutoMigrate(db); err != nil {
		logger.Error("Failed to migrate database", slog.String("error", err.Error()))
		log.Fatalf("Failed to migrate database: %v", err)
	}

	// 初始化默认数据
	if err := models.Seed(db); err != nil {
		logger.Error("Failed to seed database", slog.String("error", err.Error()))
		log.Fatalf("Failed to seed database: %v", err)
	}

	logger.Info("Database connected and migrated successfully")

	// 初始化全局设置（带缓存）
	services.InitSettings(db)

	// 初始化 i18n
	if err := services.InitI18n("locales"); err != nil {
		logger.Error("Failed to init i18n", slog.String("error", err.Error()))
		log.Fatalf("Failed to init i18n: %v", err)
	}

	// 初始化服务
	jwtService := services.NewJWTService(&cfg.JWT)
	authService := services.NewAuthService(db, jwtService)

	// 初始化中间件
	authMiddleware := middleware.NewAuthMiddleware(jwtService)

	// 初始化处理器
	authHandler := handlers.NewAuthHandler(authService)

	app := fiber.New(fiber.Config{
		AppName: "RikkaHub Server",
	})

	// Middleware
	app.Use(recover.New())
	app.Use(logging.FiberMiddleware(logger)) // 使用 slog 中间件
	app.Use(cors.New())
	app.Use(middleware.I18n())

	// Health check
	app.Get("/health", func(c *fiber.Ctx) error {
		return c.JSON(fiber.Map{
			"status": "ok",
		})
	})

	// 注册路由
	authHandler.RegisterRoutes(app, authMiddleware)

	// Start server
	logger.Info("Server starting", slog.String("port", cfg.Server.Port))
	log.Fatal(app.Listen(":" + cfg.Server.Port))
}
