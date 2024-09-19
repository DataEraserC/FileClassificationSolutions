package service

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

func Ping(c *gin.Context) {
	c.JSON(http.StatusOK, gin.H{
		"status":  "ok",
		"code":    200,
		"message": "pong",
	})
}
