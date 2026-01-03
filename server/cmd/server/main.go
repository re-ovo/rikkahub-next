package main

import (
	"log"

	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/fiber/v2/middleware/cors"
	"github.com/gofiber/fiber/v2/middleware/logger"
	"github.com/gofiber/fiber/v2/middleware/recover"
	"github.com/reovo/rikkahub/server/internal/config"
	"github.com/reovo/rikkahub/server/internal/database"
	"github.com/reovo/rikkahub/server/internal/handlers"
	"github.com/reovo/rikkahub/server/internal/middleware"
	"github.com/reovo/rikkahub/server/internal/models"
	"github.com/reovo/rikkahub/server/internal/services"
)

func main() {
	// 加载配置
	cfg, err := config.Load()
	if err != nil {
		log.Fatalf("Failed to load config: %v", err)
	}

	// 连接数据库
	db, err := database.New(&cfg.Database)
	if err != nil {
		log.Fatalf("Failed to connect database: %v", err)
	}

	// 自动迁移
	if err := models.AutoMigrate(db); err != nil {
		log.Fatalf("Failed to migrate database: %v", err)
	}

	// 初始化默认数据
	if err := models.Seed(db); err != nil {
		log.Fatalf("Failed to seed database: %v", err)
	}

	log.Println("Database connected and migrated successfully")

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
	app.Use(logger.New())
	app.Use(cors.New())

	// Health check
	app.Get("/health", func(c *fiber.Ctx) error {
		return c.JSON(fiber.Map{
			"status": "ok",
		})
	})

	// 注册路由
	authHandler.RegisterRoutes(app, authMiddleware)

	// Start server
	log.Printf("Server starting on port %s", cfg.Server.Port)
	log.Fatal(app.Listen(":" + cfg.Server.Port))
}
