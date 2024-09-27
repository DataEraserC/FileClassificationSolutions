package models

import "gorm.io/gorm"

type Tag struct {
	gorm.Model
	ID   int64  `gorm:"primaryKey;autoIncrement;unique"`
	Name string `gorm:"not null;unique"`
}
