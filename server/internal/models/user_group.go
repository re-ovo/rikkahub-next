package models

import (
	"time"

	"github.com/google/uuid"
)

// UserGroup 用户-组关联表 (多对多中间表)
type UserGroup struct {
	UserID    uuid.UUID `gorm:"type:uuid;primaryKey" json:"user_id"`
	GroupID   uuid.UUID `gorm:"type:uuid;primaryKey" json:"group_id"`
	CreatedAt time.Time `json:"created_at"`
}

func (UserGroup) TableName() string {
	return "user_groups"
}
