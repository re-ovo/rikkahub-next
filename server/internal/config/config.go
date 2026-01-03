package config

import (
	"os"
	"strconv"
	"time"

	"github.com/joho/godotenv"
)

type Config struct {
	Server   ServerConfig
	Database DatabaseConfig
	JWT      JWTConfig
	Debug    bool
}

type ServerConfig struct {
	Port string
}

type DatabaseConfig struct {
	DSN string
}

type JWTConfig struct {
	Secret           string
	AccessExpiresIn  time.Duration
	RefreshExpiresIn time.Duration
}

func Load() (*Config, error) {
	_ = godotenv.Load() // 忽略错误，允许没有 .env 文件

	return &Config{
		Server: ServerConfig{
			Port: getEnv("SERVER_PORT", "3000"),
		},
		Database: DatabaseConfig{
			DSN: getEnv("DATABASE_URL", "postgres://postgres@localhost:5432/rikkahub?sslmode=disable"),
		},
		JWT: JWTConfig{
			Secret:           getEnv("JWT_SECRET", "your-secret-key-change-in-production"),
			AccessExpiresIn:  time.Duration(getEnvInt("JWT_ACCESS_EXPIRES_HOURS", 24)) * time.Hour,
			RefreshExpiresIn: time.Duration(getEnvInt("JWT_REFRESH_EXPIRES_DAYS", 7)) * 24 * time.Hour,
		},
		Debug: getEnv("DEBUG", "false") == "true",
	}, nil
}

func getEnv(key, defaultValue string) string {
	if value, exists := os.LookupEnv(key); exists {
		return value
	}
	return defaultValue
}

func getEnvInt(key string, defaultValue int) int {
	if value, exists := os.LookupEnv(key); exists {
		if intValue, err := strconv.Atoi(value); err == nil {
			return intValue
		}
	}
	return defaultValue
}
