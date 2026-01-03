package logging

import (
	"context"
	"errors"
	"log/slog"
	"time"

	"gorm.io/gorm"
	gormlogger "gorm.io/gorm/logger"
)

// GormLogger 适配 GORM 到 slog
type GormLogger struct {
	logger        *slog.Logger
	level         gormlogger.LogLevel
	slowThreshold time.Duration
}

// NewGormLogger 创建 GORM logger
func NewGormLogger(logger *slog.Logger, level gormlogger.LogLevel) *GormLogger {
	return &GormLogger{
		logger:        logger,
		level:         level,
		slowThreshold: 200 * time.Millisecond,
	}
}

func (l *GormLogger) LogMode(level gormlogger.LogLevel) gormlogger.Interface {
	newLogger := *l
	newLogger.level = level
	return &newLogger
}

func (l *GormLogger) Info(ctx context.Context, msg string, data ...interface{}) {
	if l.level >= gormlogger.Info {
		l.logger.InfoContext(ctx, msg, "data", data)
	}
}

func (l *GormLogger) Warn(ctx context.Context, msg string, data ...interface{}) {
	if l.level >= gormlogger.Warn {
		l.logger.WarnContext(ctx, msg, "data", data)
	}
}

func (l *GormLogger) Error(ctx context.Context, msg string, data ...interface{}) {
	if l.level >= gormlogger.Error {
		l.logger.ErrorContext(ctx, msg, "data", data)
	}
}

func (l *GormLogger) Trace(ctx context.Context, begin time.Time, fc func() (string, int64), err error) {
	if l.level <= gormlogger.Silent {
		return
	}

	elapsed := time.Since(begin)
	sql, rows := fc()

	attrs := []any{
		slog.String("sql", sql),
		slog.Int64("rows", rows),
		slog.Duration("elapsed", elapsed),
	}

	switch {
	case err != nil && l.level >= gormlogger.Error && !errors.Is(err, gorm.ErrRecordNotFound):
		l.logger.ErrorContext(ctx, "database error", append(attrs, slog.String("error", err.Error()))...)
	case elapsed > l.slowThreshold && l.level >= gormlogger.Warn:
		l.logger.WarnContext(ctx, "slow query", attrs...)
	case l.level >= gormlogger.Info:
		l.logger.DebugContext(ctx, "query", attrs...)
	}
}
