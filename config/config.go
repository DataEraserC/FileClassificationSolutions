package config

import (
	"bufio"
	"encoding/json"
	"log"
	"os"
)

type Config struct {
	// AppName     string `json:"app_name"`
	// AppModel    string `json:"app_model"`
	AppHost string `json:"app_host"`
	AppPort string `json:"app_port"`
	Path    string `json:path`
	// AppId       string `json:"app_id"`
	// AppSecret   string `json:"app_secret"`
	// PicturesDir string `json:"pictures_dir"`
	// AvatarDir   string `json:"avatar_dir"`
	// QRCodePath  string `json:"qrcode_path"`
	// ExcelPath   string `json:"excel_path"`
	Database DatabaseConfig `json:"database"`
	// Jwt JwtConfig `json:"jwt"`
}

type DatabaseConfig struct {
	Driver          string `json:"driver"`
	User            string `json:"user"`
	Password        string `json:"password"`
	Host            string `json:"host"`
	Port            string `json:"port"`
	DbName          string `json:"db_name"`
	Charset         string `json:"charset"`
	ParseTime       bool   `json:"parse_time"`
	Loc             string `json:"loc"`
	MaxIdleConn     int    `json:"max_idle_connections"`
	MaxOpenConn     int    `json:"max_open_connections"`
	ConnMaxLifeTime int    `json:"connection_max_lifetime"`
}

type JwtConfig struct {
	PublicKeyPath   string `json:"public_key_path"`
	PrivateKeyPath  string `json:"private_key_path"`
	ExpirationDates int    `json:"expiration_dates"`
}

// Cfg 全局配置变量
var Cfg *Config = parseConfig()

// parseConfig 函数用于加载配置
func parseConfig() *Config {
	var cfg *Config = nil
	file, err := os.Open("./config/config.json")
	defer func(file *os.File) {
		err := file.Close()
		if err != nil {
			log.Print("Failed to close file: " + err.Error())
			panic(err.Error())
		}
	}(file)

	if err != nil {
		log.Print(err.Error())
		panic(err)
	}
	reader := bufio.NewReader(file)
	decoder := json.NewDecoder(reader)
	if err = decoder.Decode(&cfg); err != nil {
		log.Print("Decode error:" + err.Error())
		panic(err)
	}
	return cfg
}
