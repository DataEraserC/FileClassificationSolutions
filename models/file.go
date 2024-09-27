package models

import "gorm.io/gorm"

type File struct {
	gorm.Model
	ID         int64  `gorm:"primaryKey;autoIncrement;unique"`
	Type       string `gorm:"not null"`
	Location   string `gorm:"not null"`
	Name       string `gorm:"not null"`
	ClickCount int64  `gorm:"default:0"`
	ShareCount int64  `gorm:"default:0"`
}
