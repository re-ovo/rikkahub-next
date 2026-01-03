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
}

type ServerConfig struct {
	Port string
}

type DatabaseConfig struct {
	Host     string
	Port     string
	User     string
	Password string
	DBName   string
	SSLMode  string
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
			Host:     getEnv("DB_HOST", "localhost"),
			Port:     getEnv("DB_PORT", "5432"),
			User:     getEnv("DB_USER", "postgres"),
			Password: getEnv("DB_PASSWORD", ""),
			DBName:   getEnv("DB_NAME", "rikkahub"),
			SSLMode:  getEnv("DB_SSLMODE", "disable"),
		},
		JWT: JWTConfig{
			Secret:           getEnv("JWT_SECRET", "your-secret-key-change-in-production"),
			AccessExpiresIn:  time.Duration(getEnvInt("JWT_ACCESS_EXPIRES_HOURS", 24)) * time.Hour,
			RefreshExpiresIn: time.Duration(getEnvInt("JWT_REFRESH_EXPIRES_DAYS", 7)) * 24 * time.Hour,
		},
	}, nil
}

func (c *DatabaseConfig) DSN() string {
	return "host=" + c.Host +
		" user=" + c.User +
		" password=" + c.Password +
		" dbname=" + c.DBName +
		" port=" + c.Port +
		" sslmode=" + c.SSLMode
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
