package logging

import (
	"log/slog"
	"os"
	"time"

	"github.com/lmittmann/tint"
)

var Logger *slog.Logger

// Init 初始化全局 logger
func Init(debug bool) {
	level := slog.LevelInfo
	if debug {
		level = slog.LevelDebug
	}

	// 使用 tint 提供带颜色的美化输出
	handler := tint.NewHandler(os.Stdout, &tint.Options{
		Level:      level,
		TimeFormat: time.TimeOnly, // 只显示时间，不显示日期
		AddSource:  debug,         // debug 模式显示源码位置
	})

	Logger = slog.New(handler)
	slog.SetDefault(Logger)
}

// GetLogger 获取全局 logger
func GetLogger() *slog.Logger {
	if Logger == nil {
		Init(false)
	}
	return Logger
}
