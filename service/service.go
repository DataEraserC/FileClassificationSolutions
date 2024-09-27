package service

import (
	"file-management-system/logic"
	"file-management-system/models"
	"file-management-system/typings"
	"net/http"
	"strconv"

	"github.com/gin-gonic/gin"
)

// CreateFileHandler 创建文件记录的 Gin handler
func CreateFileHandler(c *gin.Context) {
	var req typings.CreateFileRequest
	var file models.File
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}
	if req.Name != nil {
		file.Name = *req.Name
	}
	if req.Type != nil {
		file.Type = *req.Type
	}
	id, err := logic.CreateFile(&file)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, gin.H{"id": id})
}

// CreateTagHandler 创建标签记录的 Gin handler
func CreateTagHandler(c *gin.Context) {
	var req typings.CreateTagRequest
	var tag models.Tag
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}
	if req.Name != nil {
		tag.Name = *req.Name
	}
	id, err := logic.CreateTag(&tag)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, gin.H{"id": id})
}

// CreateFileTagRelationHandler 创建文件和标签关系的 Gin handler
func CreateFileTagRelationHandler(c *gin.Context) {
	var req typings.CreateFileTagRelationRequest
	var fileTag models.FileTag
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}
	if req.FileID != nil {
		fileTag.FileID = *req.FileID
	}
	if req.TagID != nil {
		fileTag.TagID = *req.TagID
	}

	err := logic.CreateFileTagRelation(&fileTag)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, gin.H{"message": "Relation created successfully"})
}

// TODO: FindFileHandler 查找文件记录的 Gin handler
func FindFileHandler(c *gin.Context) {
	fileID, err := strconv.ParseInt(c.Param("file_id"), 10, 64)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Invalid file_id"})
		return
	}
	file := models.File{ID: fileID}
	foundFile, err := logic.FindFile(&file)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}
	if foundFile == nil {
		c.JSON(http.StatusNotFound, gin.H{"error": "File not found"})
		return
	}

	c.JSON(http.StatusOK, foundFile)
}

// FindFileByFuzzyNameHandler 根据文件名模糊搜索文件记录的 Gin handler
func FindFileByFuzzyNameHandler(c *gin.Context) {
	fuzzyName := c.Query("fuzzy_name")
	files, err := logic.FindFileByFuzzyName(fuzzyName)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}
	if files == nil {
		c.JSON(http.StatusNotFound, gin.H{"error": "Files not found"})
		return
	}

	c.JSON(http.StatusOK, files)
}

// FindFileByExactNameHandler 根据文件名精确搜索文件记录的 Gin handler
func FindFileByExactNameHandler(c *gin.Context) {
	exactName := c.Query("exact_name")
	file, err := logic.FindFileByExactName(exactName)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}
	if file == nil {
		c.JSON(http.StatusNotFound, gin.H{"error": "File not found"})
		return
	}

	c.JSON(http.StatusOK, file)
}

// FindFilesByTagNameHandler 查找具有特定标签的所有文件记录的 Gin handler
func FindFilesByTagNameHandler(c *gin.Context) {
	tagName := c.Query("tag_name")
	files, err := logic.FindFilesByTagName(tagName)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}
	if files == nil {
		c.JSON(http.StatusNotFound, gin.H{"error": "Files not found"})
		return
	}

	c.JSON(http.StatusOK, files)
}

// FindFilesByTagIDHandler 查找具有特定标签ID的所有文件记录的 Gin handler
func FindFilesByTagIDHandler(c *gin.Context) {
	tagID, err := strconv.ParseInt(c.Query("tag_id"), 10, 64)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Invalid tag_id"})
		return
	}
	files, err := logic.FindFilesByTagID(tagID)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}
	if files == nil {
		c.JSON(http.StatusNotFound, gin.H{"error": "Files not found"})
		return
	}

	c.JSON(http.StatusOK, files)
}

// FindTagHandler 根据请求的标签信息查找标签的 Gin handler
func FindTagHandler(c *gin.Context) {
	req := &models.Tag{}
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}
	tag, err := logic.FindTag(req)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}
	if tag == nil {
		c.JSON(http.StatusNotFound, gin.H{"error": "Tag not found"})
		return
	}

	c.JSON(http.StatusOK, tag)
}

// FindTagByFuzzyNameHandler 根据标签名进行模糊搜索的 Gin handler
func FindTagByFuzzyNameHandler(c *gin.Context) {
	fuzzyName := c.Query("fuzzy_name")
	tags, err := logic.FindTagByFuzzyName(fuzzyName)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}
	if tags == nil {
		c.JSON(http.StatusNotFound, gin.H{"error": "Tags not found"})
		return
	}

	c.JSON(http.StatusOK, tags)
}

// FindTagByExactNameHandler 根据标签名进行精确搜索的 Gin handler
func FindTagByExactNameHandler(c *gin.Context) {
	exactName := c.Query("exact_name")
	tag, err := logic.FindTagByExactName(exactName)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}
	if tag == nil {
		c.JSON(http.StatusNotFound, gin.H{"error": "Tag not found"})
		return
	}

	c.JSON(http.StatusOK, tag)
}

// FindTagsByFileHandler 查找与特定文件相关联的所有标签的 Gin handler
func FindTagsByFileHandler(c *gin.Context) {
	fileID, err := strconv.ParseInt(c.Param("file_id"), 10, 64)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Invalid file_id"})
		return
	}
	tags, err := logic.FindTagsByFile(fileID)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}
	if tags == nil {
		c.JSON(http.StatusNotFound, gin.H{"error": "Tags not found"})
		return
	}

	c.JSON(http.StatusOK, tags)
}

// UpdateFileHandler 更新文件记录的 Gin handler
func UpdateFileHandler(c *gin.Context) {
	fileID, err := strconv.ParseInt(c.Query("file_id"), 10, 64)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Invalid file_id"})
		return
	}
	file := &models.File{ID: fileID}
	if err := c.ShouldBindJSON(&file); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	err = logic.UpdateFile(file)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, gin.H{"message": "File updated successfully"})
}

// UpdateTagHandler 更新标签记录的 Gin handler
func UpdateTagHandler(c *gin.Context) {
	tagID, err := strconv.ParseInt(c.Query("tag_id"), 10, 64)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Invalid tag_id"})
		return
	}
	tag := &models.Tag{ID: tagID}
	if err := c.ShouldBindJSON(&tag); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": err.Error()})
		return
	}

	err = logic.UpdateTag(tag)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, gin.H{"message": "Tag updated successfully"})
}

// DeleteFileHandler 删除文件记录及其与标签的关系的 Gin handler
func DeleteFileHandler(c *gin.Context) {
	fileID, err := strconv.ParseInt(c.Query("file_id"), 10, 64)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Invalid file_id"})
		return
	}
	err = logic.DeleteFile(fileID)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, gin.H{"message": "File deleted successfully"})
}

// DeleteTagHandler 删除标签记录及其与文件的关系的 Gin handler
func DeleteTagHandler(c *gin.Context) {
	tagID, err := strconv.ParseInt(c.Query("tag_id"), 10, 64)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Invalid tag_id"})
		return
	}
	err = logic.DeleteTag(tagID)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, gin.H{"message": "Tag deleted successfully"})
}

// DeleteFileTagRelationHandler 删除文件和标签之间的关联关系的 Gin handler
func DeleteFileTagRelationHandler(c *gin.Context) {

	fileID, err := strconv.ParseInt(c.Query("file_id"), 10, 64)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Invalid file_id"})
		return
	}
	tagID, err := strconv.ParseInt(c.Query("tag_id"), 10, 64)
	if err != nil {
		c.JSON(http.StatusBadRequest, gin.H{"error": "Invalid tag_id"})
		return
	}
	err = logic.DeleteFileTagRelation(fileID, tagID)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}

	c.JSON(http.StatusOK, gin.H{"message": "Relation deleted successfully"})
}
