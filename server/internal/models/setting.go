package models

import (
	"time"

	"gorm.io/datatypes"
)

type SettingType string

const (
	SettingTypeBool   SettingType = "bool"
	SettingTypeString SettingType = "string"
	SettingTypeInt    SettingType = "int"
	SettingTypeJSON   SettingType = "json"
)

type Setting struct {
	Key         string         `gorm:"type:varchar(100);primaryKey" json:"key"`
	Value       datatypes.JSON `gorm:"type:jsonb;not null" json:"value"`
	Type        SettingType    `gorm:"type:varchar(20);not null" json:"type"`
	Description string         `gorm:"type:varchar(255)" json:"description"`
	UpdatedAt   time.Time      `json:"updated_at"`
}

func (Setting) TableName() string {
	return "settings"
}
