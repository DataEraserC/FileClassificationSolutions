package models

import (
	"file-management-system/config"
	"log"
	"strconv"
	"time"

	"gorm.io/driver/mysql"
	"gorm.io/driver/sqlite"
	"gorm.io/gorm"
)

type UnsupportedDatabaseError struct {
	DriverName string
}

func (e UnsupportedDatabaseError) Error() string {
	return "Unsupported database driver: " + e.DriverName
}
func Database() (db *gorm.DB, err error) {
	switch config.Cfg.Database.Driver {
	case "sqlite":
		dsn := "file:" + config.Cfg.Database.DbName + "?cache=shared&mode=rwc"
		db, err := gorm.Open(sqlite.Open(dsn), &gorm.Config{})
		sqlDB, err := db.DB()
		sqlDB.SetMaxIdleConns(config.Cfg.Database.MaxIdleConn)
		sqlDB.SetMaxOpenConns(config.Cfg.Database.MaxOpenConn)
		sqlDB.SetConnMaxLifetime(time.Duration(config.Cfg.Database.ConnMaxLifeTime) * time.Second)
		return db, err
	case "mysql":
		dsn := config.Cfg.Database.User + ":" + config.Cfg.Database.Password + "@tcp(" + config.Cfg.Database.Host + ":" + config.Cfg.Database.Port + ")/" + config.Cfg.Database.DbName + "?charset=" + config.Cfg.Database.Charset + "&parseTime=" + strconv.FormatBool(config.Cfg.Database.ParseTime) + "&loc=" + config.Cfg.Database.Loc
		db, err := gorm.Open(mysql.Open(dsn), &gorm.Config{})
		sqlDB, err := db.DB()
		sqlDB.SetMaxIdleConns(config.Cfg.Database.MaxIdleConn)
		sqlDB.SetMaxOpenConns(config.Cfg.Database.MaxOpenConn)
		sqlDB.SetConnMaxLifetime(time.Duration(config.Cfg.Database.ConnMaxLifeTime) * time.Second)
		return db, err
	default:
		err := UnsupportedDatabaseError{DriverName: config.Cfg.Database.Driver}
		log.Fatal(err.Error())
		return nil, err
	}

}
