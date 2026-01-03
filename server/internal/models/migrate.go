package models

import (
	"gorm.io/gorm"
)

// AllModels 返回所有需要迁移的模型
func AllModels() []interface{} {
	return []interface{}{
		&User{},
		&OAuthAccount{},
		&Group{},
		&GroupPermission{},
		&UserGroup{},
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

	return nil
}
