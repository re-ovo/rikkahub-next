package models

import (
	"time"

	"github.com/google/uuid"
	"gorm.io/gorm"
)

type GroupPermission struct {
	ID         uuid.UUID      `gorm:"type:uuid;primaryKey" json:"id"`
	GroupID    uuid.UUID      `gorm:"type:uuid;not null;index" json:"group_id"`
	Permission string         `gorm:"type:varchar(100);not null" json:"permission"`
	CreatedAt  time.Time      `json:"created_at"`
	DeletedAt  gorm.DeletedAt `gorm:"index" json:"-"`

	// 关联
	Group Group `gorm:"foreignKey:GroupID" json:"group,omitempty"`
}

func (p *GroupPermission) BeforeCreate(tx *gorm.DB) error {
	if p.ID == uuid.Nil {
		p.ID = uuid.New()
	}
	return nil
}

func (GroupPermission) TableName() string {
	return "group_permissions"
}
