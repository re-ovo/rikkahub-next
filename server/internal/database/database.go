package database

import (
	"github.com/reovo/rikkahub/server/internal/config"
	"github.com/reovo/rikkahub/server/pkg/logging"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
	"gorm.io/gorm/logger"
)

func New(cfg *config.DatabaseConfig, debug bool) (*gorm.DB, error) {
	// 根据 debug 模式设置日志级别
	logLevel := logger.Warn // 生产环境只显示警告和错误
	if debug {
		logLevel = logger.Info // 开发环境显示所有 SQL
	}

	gormLogger := logging.NewGormLogger(logging.GetLogger(), logLevel)

	db, err := gorm.Open(postgres.Open(cfg.DSN), &gorm.Config{
		Logger: gormLogger,
	})
	if err != nil {
		return nil, err
	}

	return db, nil
}
