package models

import (
	"time"

	"github.com/google/uuid"
	"gorm.io/gorm"
)

type UserStatus int8

const (
	UserStatusDisabled UserStatus = 0
	UserStatusActive   UserStatus = 1
)

type User struct {
	ID           uuid.UUID      `gorm:"type:uuid;primaryKey" json:"id"`
	Username     *string        `gorm:"type:varchar(50);uniqueIndex" json:"username"`
	Email        string         `gorm:"type:varchar(255);uniqueIndex;not null" json:"email"`
	PasswordHash *string        `gorm:"type:varchar(255)" json:"-"`
	Nickname     string         `gorm:"type:varchar(100)" json:"nickname"`
	Avatar       string         `gorm:"type:varchar(500)" json:"avatar"`
	Status       UserStatus     `gorm:"type:smallint;default:1" json:"status"`
	CreatedAt    time.Time      `json:"created_at"`
	UpdatedAt    time.Time      `json:"updated_at"`
	DeletedAt    gorm.DeletedAt `gorm:"index" json:"-"`

	// 关联
	OAuthAccounts []OAuthAccount `gorm:"foreignKey:UserID" json:"oauth_accounts,omitempty"`
	Groups        []Group        `gorm:"many2many:user_groups" json:"groups,omitempty"`
}

func (u *User) BeforeCreate(tx *gorm.DB) error {
	if u.ID == uuid.Nil {
		u.ID = uuid.New()
	}
	return nil
}

// TableName 指定表名
func (User) TableName() string {
	return "users"
}
