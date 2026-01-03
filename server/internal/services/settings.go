package services

import (
	"encoding/json"
	"sync"
	"time"

	"github.com/reovo/rikkahub/server/internal/models"
	"gorm.io/datatypes"
	"gorm.io/gorm"
)

// Settings 全局设置访问器（带缓存）
type Settings struct {
	db    *gorm.DB
	cache map[string]*models.Setting
	mu    sync.RWMutex
	ttl   time.Duration
	lastLoad time.Time
}

var (
	globalSettings *Settings
	settingsOnce   sync.Once
)

// InitSettings 初始化全局设置（启动时调用一次）
func InitSettings(db *gorm.DB) *Settings {
	settingsOnce.Do(func() {
		globalSettings = &Settings{
			db:    db,
			cache: make(map[string]*models.Setting),
			ttl:   5 * time.Minute, // 缓存 5 分钟
		}
		globalSettings.Reload()
	})
	return globalSettings
}

// GetSettings 获取全局设置实例
func GetSettings() *Settings {
	return globalSettings
}

// Reload 重新加载所有设置到缓存
func (s *Settings) Reload() error {
	var settings []models.Setting
	if err := s.db.Find(&settings).Error; err != nil {
		return err
	}

	s.mu.Lock()
	defer s.mu.Unlock()

	s.cache = make(map[string]*models.Setting)
	for i := range settings {
		s.cache[settings[i].Key] = &settings[i]
	}
	s.lastLoad = time.Now()
	return nil
}

// checkAndReload 检查缓存是否过期，过期则重新加载
func (s *Settings) checkAndReload() {
	s.mu.RLock()
	expired := time.Since(s.lastLoad) > s.ttl
	s.mu.RUnlock()

	if expired {
		s.Reload()
	}
}

// get 从缓存获取设置
func (s *Settings) get(key string) *models.Setting {
	s.checkAndReload()

	s.mu.RLock()
	defer s.mu.RUnlock()
	return s.cache[key]
}

// GetBool 获取布尔值
func (s *Settings) GetBool(key string, defaultValue bool) bool {
	setting := s.get(key)
	if setting == nil {
		return defaultValue
	}
	var value bool
	if err := json.Unmarshal(setting.Value, &value); err != nil {
		return defaultValue
	}
	return value
}

// GetString 获取字符串
func (s *Settings) GetString(key string, defaultValue string) string {
	setting := s.get(key)
	if setting == nil {
		return defaultValue
	}
	var value string
	if err := json.Unmarshal(setting.Value, &value); err != nil {
		return defaultValue
	}
	return value
}

// GetInt 获取整数
func (s *Settings) GetInt(key string, defaultValue int) int {
	setting := s.get(key)
	if setting == nil {
		return defaultValue
	}
	var value int
	if err := json.Unmarshal(setting.Value, &value); err != nil {
		return defaultValue
	}
	return value
}

// Set 设置值（同时更新缓存和数据库）
func (s *Settings) Set(key string, value any, settingType models.SettingType, description string) error {
	jsonValue, err := json.Marshal(value)
	if err != nil {
		return err
	}

	setting := &models.Setting{
		Key:         key,
		Value:       datatypes.JSON(jsonValue),
		Type:        settingType,
		Description: description,
	}

	if err := s.db.Save(setting).Error; err != nil {
		return err
	}

	// 更新缓存
	s.mu.Lock()
	s.cache[key] = setting
	s.mu.Unlock()

	return nil
}

// SetBool 设置布尔值
func (s *Settings) SetBool(key string, value bool) error {
	return s.Set(key, value, models.SettingTypeBool, "")
}

// SetString 设置字符串
func (s *Settings) SetString(key string, value string) error {
	return s.Set(key, value, models.SettingTypeString, "")
}

// SetInt 设置整数
func (s *Settings) SetInt(key string, value int) error {
	return s.Set(key, value, models.SettingTypeInt, "")
}

// ============ 便捷访问方法 ============

// Auth 认证相关设置
func (s *Settings) Auth() *AuthSettings {
	return &AuthSettings{s}
}

// Chat 聊天相关设置
func (s *Settings) Chat() *ChatSettings {
	return &ChatSettings{s}
}

// AuthSettings 认证设置
type AuthSettings struct{ s *Settings }

func (a *AuthSettings) AllowRegister() bool {
	return a.s.GetBool("auth.allow_register", true)
}

func (a *AuthSettings) AllowOAuth() bool {
	return a.s.GetBool("auth.allow_oauth", true)
}

func (a *AuthSettings) DefaultGroup() string {
	return a.s.GetString("auth.default_group", "user")
}

// ChatSettings 聊天设置
type ChatSettings struct{ s *Settings }

func (c *ChatSettings) MaxContextMessages() int {
	return c.s.GetInt("chat.max_context_messages", 50)
}

func (c *ChatSettings) DefaultModel() string {
	return c.s.GetString("chat.default_model", "gpt-4")
}
