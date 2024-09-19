package main

import (
	"file-management-system/config"
	"log"

	"file-management-system/router"

	"github.com/gin-gonic/gin"
)

func main() {

	r := router.Router()

	gin.SetMode(gin.ReleaseMode)

	runErr := r.Run(":" + config.Cfg.AppPort)
	if runErr != nil {
		log.Println("Runtime error: " + runErr.Error())
		panic("Runtime error: " + runErr.Error())
	}
}
