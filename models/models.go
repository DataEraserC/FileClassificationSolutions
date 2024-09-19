package models

import (
	"log"

	"gorm.io/gorm"
)

var DB = Init()

// Init 初始化数据库链接
func Init() *gorm.DB {
	db, err := Database()
	if err != nil {
		log.Fatal("failed to connect database:" + err.Error())
	}

	db.AutoMigrate(&File{}, &Tag{}, &FileTag{})

	return db
}
