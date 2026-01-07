package models

import (
	"encoding/json"

	"github.com/reovo/rikkahub/server/pkg/crypto"
	"gorm.io/datatypes"
	"gorm.io/gorm"
)

// AllModels 返回所有需要迁移的模型
func AllModels() []interface{} {
	return []interface{}{
		&User{},
		&Group{},
		&GroupPermission{},
		&UserGroup{},
		&Setting{},
	}
}

// AutoMigrate 自动迁移所有模型
func AutoMigrate(db *gorm.DB) error {
	return db.AutoMigrate(AllModels()...)
}

// Seed 初始化默认数据
func Seed(db *gorm.DB) error {
	// 默认用户组
	groups := []struct {
		Name        string
		Description string
		IsDefault   bool
		Permissions []string
	}{
		{
			Name:        "guest",
			Description: "访客用户组",
			IsDefault:   false,
			Permissions: []string{
				"conversation.create",
				"conversation.read",
				"model.gpt-3.5.use",
			},
		},
		{
			Name:        "user",
			Description: "普通用户组",
			IsDefault:   true,
			Permissions: []string{
				"conversation.*",
				"model.*.use",
			},
		},
		{
			Name:        "admin",
			Description: "管理员用户组",
			IsDefault:   false,
			Permissions: []string{
				"*",
			},
		},
	}

	for _, g := range groups {
		var group Group
		result := db.Where("name = ?", g.Name).First(&group)
		if result.Error == gorm.ErrRecordNotFound {
			group = Group{
				Name:        g.Name,
				Description: g.Description,
				IsDefault:   g.IsDefault,
			}
			if err := db.Create(&group).Error; err != nil {
				return err
			}

			// 创建权限
			for _, perm := range g.Permissions {
				permission := GroupPermission{
					GroupID:    group.ID,
					Permission: perm,
				}
				if err := db.Create(&permission).Error; err != nil {
					return err
				}
			}
		}
	}

	// 默认管理员用户
	if err := seedDefaultAdmin(db); err != nil {
		return err
	}

	// 默认设置
	if err := seedSettings(db); err != nil {
		return err
	}

	return nil
}

// seedSettings 初始化默认设置
func seedSettings(db *gorm.DB) error {
	settings := []struct {
		Key         string
		Value       interface{}
		Type        SettingType
		Description string
	}{
		{"auth.allow_register", true, SettingTypeBool, "是否允许新用户注册"},
		{"auth.default_group", "user", SettingTypeString, "新用户默认用户组"},
		{"chat.max_context_messages", 50, SettingTypeInt, "聊天最大上下文消息数"},
		{"chat.default_model", "gpt-4", SettingTypeString, "默认 AI 模型"},
	}

	for _, s := range settings {
		var existing Setting
		if err := db.Where("key = ?", s.Key).First(&existing).Error; err == gorm.ErrRecordNotFound {
			jsonValue, err := json.Marshal(s.Value)
			if err != nil {
				return err
			}
			setting := Setting{
				Key:         s.Key,
				Value:       datatypes.JSON(jsonValue),
				Type:        s.Type,
				Description: s.Description,
			}
			if err := db.Create(&setting).Error; err != nil {
				return err
			}
		}
	}

	return nil
}

// seedDefaultAdmin 创建默认管理员用户
func seedDefaultAdmin(db *gorm.DB) error {
	var adminUser User
	// 检查是否已存在 admin 用户
	if err := db.Where("username = ?", "admin").First(&adminUser).Error; err == gorm.ErrRecordNotFound {
		// 哈希密码
		passwordHash, err := crypto.HashPassword("admin")
		if err != nil {
			return err
		}

		// 创建管理员用户
		adminUser = User{
			Username:     "admin",
			PasswordHash: &passwordHash,
			Nickname:     "系统管理员",
			Avatar:       "",
			Status:       UserStatusActive,
		}
		if err := db.Create(&adminUser).Error; err != nil {
			return err
		}

		// 获取 admin 用户组
		var adminGroup Group
		if err := db.Where("name = ?", "admin").First(&adminGroup).Error; err != nil {
			return err
		}

		// 将用户添加到 admin 组
		userGroup := UserGroup{
			UserID:  adminUser.ID,
			GroupID: adminGroup.ID,
		}
		if err := db.Create(&userGroup).Error; err != nil {
			return err
		}
	}

	return nil
}
