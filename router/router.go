package router

import (
	"file-management-system/service"

	"github.com/gin-gonic/gin"
)

func Router() *gin.Engine {
	router := gin.Default()
	v1 := router.Group("/api/v1")
	{
		v1.GET("/ping", service.Ping)
		v1.POST("/create_file", service.CreateFileHandler)
		v1.POST("/create_tag", service.CreateTagHandler)
		v1.POST("/create_file_tag_relation", service.CreateFileTagRelationHandler)
		v1.POST("/find_file", service.FindFileHandler)
		v1.POST("/find_file_by_fuzzy_name", service.FindFileByFuzzyNameHandler)
		v1.POST("/find_file_by_exact_name", service.FindFileByExactNameHandler)
		v1.POST("/find_file_by_tag_name", service.FindFilesByTagNameHandler)
		v1.POST("/find_file_by_tag_id", service.FindFilesByTagIDHandler)
		v1.POST("/find_tag", service.FindTagHandler)
		v1.POST("/find_tag_by_fuzzy_name", service.FindTagByFuzzyNameHandler)
		v1.POST("/find_tag_by_exact_name", service.FindTagByExactNameHandler)
		v1.POST("/find_tag_by_file_id", service.FindTagsByFileHandler)
		v1.POST("/update_file", service.UpdateFileHandler)
		v1.POST("/update_tag", service.UpdateTagHandler)
		v1.POST("/delete_file", service.DeleteFileHandler)
		v1.POST("/delete_tag", service.DeleteTagHandler)
		v1.POST("/delete_file_tag_relation", service.DeleteFileTagRelationHandler)
		// v1.POST("/", service.)
	}
	return router
}
