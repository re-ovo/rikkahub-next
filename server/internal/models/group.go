package models

import (
	"time"

	"github.com/google/uuid"
	"gorm.io/gorm"
)

type Group struct {
	ID          uuid.UUID      `gorm:"type:uuid;primaryKey" json:"id"`
	Name        string         `gorm:"type:varchar(50);uniqueIndex;not null" json:"name"`
	Description string         `gorm:"type:varchar(255)" json:"description"`
	IsDefault   bool           `gorm:"default:false" json:"is_default"`
	CreatedAt   time.Time      `json:"created_at"`
	UpdatedAt   time.Time      `json:"updated_at"`
	DeletedAt   gorm.DeletedAt `gorm:"index" json:"-"`

	// 关联
	Users       []User            `gorm:"many2many:user_groups" json:"users,omitempty"`
	Permissions []GroupPermission `gorm:"foreignKey:GroupID" json:"permissions,omitempty"`
}

func (g *Group) BeforeCreate(tx *gorm.DB) error {
	if g.ID == uuid.Nil {
		g.ID = uuid.New()
	}
	return nil
}

func (Group) TableName() string {
	return "groups"
}
