package models

import (
	"time"

	"github.com/google/uuid"
	"gorm.io/gorm"
)

type OAuthAccount struct {
	ID           uuid.UUID      `gorm:"type:uuid;primaryKey" json:"id"`
	UserID       uuid.UUID      `gorm:"type:uuid;not null;index" json:"user_id"`
	Provider     string         `gorm:"type:varchar(50);not null" json:"provider"` // github, google, etc.
	ProviderID   string         `gorm:"type:varchar(255);not null" json:"provider_id"`
	AccessToken  string         `gorm:"type:varchar(500)" json:"-"`
	RefreshToken string         `gorm:"type:varchar(500)" json:"-"`
	ExpiresAt    *time.Time     `json:"expires_at,omitempty"`
	CreatedAt    time.Time      `json:"created_at"`
	UpdatedAt    time.Time      `json:"updated_at"`
	DeletedAt    gorm.DeletedAt `gorm:"index" json:"-"`

	// 关联
	User User `gorm:"foreignKey:UserID" json:"user,omitempty"`
}

func (o *OAuthAccount) BeforeCreate(tx *gorm.DB) error {
	if o.ID == uuid.Nil {
		o.ID = uuid.New()
	}
	return nil
}

func (OAuthAccount) TableName() string {
	return "oauth_accounts"
}

// 创建联合唯一索引的迁移钩子
func (OAuthAccount) Indexes() []string {
	return []string{
		"CREATE UNIQUE INDEX IF NOT EXISTS idx_oauth_provider_id ON oauth_accounts(provider, provider_id)",
	}
}
