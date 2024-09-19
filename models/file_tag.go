package models

type FileTag struct {
	FileID int64 `gorm:"primaryKey;not null"`
	TagID  int64 `gorm:"primaryKey;not null"`
}
