package logic

import (
	"file-management-system/models"

	"gorm.io/gorm"
)

// CreateFile 创建一个新的文件记录并返回创建成功的ID
func CreateFile(file *models.File) (int64, error) {
	err := models.DB.Create(&file).Error
	if err != nil {
		return 0, err
	}
	return file.ID, nil
}

// CreateTag 创建一个新的标签记录并返回创建成功的ID
func CreateTag(tag *models.Tag) (int64, error) {
	err := models.DB.Create(&tag).Error
	if err != nil {
		return 0, err
	}
	return tag.ID, nil
}

// CreateFileTagRelation 创建文件和标签之间的关系
func CreateFileTagRelation(fileTag *models.FileTag) error {
	return models.DB.Create(&fileTag).Error
}

// FindFile 根据文件ID查找单个文件记录

func FindFile(req *models.File) (*models.File, error) {
	result := &models.File{}
	err := models.DB.Where(req).First(&result).Error
	if err != nil {
		if err == gorm.ErrRecordNotFound {
			return nil, nil // 未找到记录，返回nil而不是错误
		}
		return nil, err
	}
	return result, nil
}

// FindFileByFuzzyName 根据标签名进行模糊搜索
func FindFileByFuzzyName(fuzzyName string) ([]*models.File, error) {
	var files []*models.File
	err := models.DB.Where("name LIKE ?", "%"+fuzzyName+"%").Find(&files).Error
	if err != nil {
		return nil, err
	}
	return files, nil
}

// FindFileByExactName 根据标签名进行精确搜索
func FindFileByExactName(exactName string) (*models.File, error) {
	file := &models.File{}
	err := models.DB.Where("name = ?", exactName).First(&file).Error
	if err != nil {
		if err == gorm.ErrRecordNotFound {
			return nil, nil // 未找到记录，返回nil而不是错误
		}
		return nil, err
	}
	return file, nil
}

// FindFilesByTagName 查找具有特定标签的所有文件
func FindFilesByTagName(tagName string) ([]*models.File, error) {
	var files []*models.File
	err := models.DB.Joins("JOIN file_tags ON file_tags.file_id = files.id").
		Joins("JOIN tags ON tags.id = file_tags.tag_id").
		Where("tags.name = ?", tagName).
		Find(&files).Error
	if err != nil {
		return nil, err
	}
	return files, nil
}

// FindFilesByTagID 查找具有特定标签ID的所有文件
func FindFilesByTagID(tagID int64) ([]*models.File, error) {
	var files []*models.File
	err := models.DB.Joins("JOIN file_tags ON file_tags.file_id = files.id").
		Where("file_tags.tag_id = ?", tagID).
		Find(&files).Error
	if err != nil {
		return nil, err
	}
	return files, nil
}

// FindTag 根据请求的标签信息查找标签
func FindTag(req *models.Tag) (*models.Tag, error) {
	result := &models.Tag{}
	err := models.DB.Where(req).First(&result).Error
	if err != nil {
		if err == gorm.ErrRecordNotFound {
			return nil, nil // 未找到记录，返回nil而不是错误
		}
		return nil, err
	}
	return result, nil
}

// FindTagByFuzzyName 根据标签名进行模糊搜索
func FindTagByFuzzyName(fuzzyName string) ([]*models.Tag, error) {
	var tags []*models.Tag
	err := models.DB.Where("name LIKE ?", "%"+fuzzyName+"%").Find(&tags).Error
	if err != nil {
		return nil, err
	}
	return tags, nil
}

// FindTagByExactName 根据标签名进行精确搜索
func FindTagByExactName(exactName string) (*models.Tag, error) {
	tag := &models.Tag{}
	err := models.DB.Where("name = ?", exactName).First(&tag).Error
	if err != nil {
		if err == gorm.ErrRecordNotFound {
			return nil, nil // 未找到记录，返回nil而不是错误
		}
		return nil, err
	}
	return tag, nil
}

// FindTagsByFile 查找与特定文件相关联的所有标签
func FindTagsByFile(fileID int64) ([]*models.Tag, error) {
	var tags []*models.Tag
	err := models.DB.Joins("JOIN file_tags ON file_tags.tag_id = tags.id").
		Where("file_tags.file_id = ?", fileID).
		Find(&tags).Error
	if err != nil {
		return nil, err
	}
	return tags, nil
}

// UpdateFile 更新文件记录
func UpdateFile(file *models.File) error {
	return models.DB.Save(file).Error
}

// UpdateTag 更新标签记录
func UpdateTag(tag *models.Tag) error {
	return models.DB.Save(tag).Error
}

// DeleteFile 删除文件记录及其与标签的关系
func DeleteFile(fileID int64) error {
	tx := models.DB.Begin()
	defer func() {
		if r := recover(); r != nil {
			tx.Rollback()
		}
	}()

	if err := tx.Delete(&models.FileTag{}, "file_id = ?", fileID).Error; err != nil {
		tx.Rollback()
		return err
	}

	if err := tx.Delete(&models.File{}, "id = ?", fileID).Error; err != nil {
		tx.Rollback()
		return err
	}

	return tx.Commit().Error
}

// DeleteTag 删除标签记录及其与文件的关系
func DeleteTag(tagID int64) error {
	tx := models.DB.Begin()
	defer func() {
		if r := recover(); r != nil {
			tx.Rollback()
		}
	}()

	// 首先删除与标签关联的文件标签关系
	if err := tx.Delete(&models.FileTag{}, "tag_id = ?", tagID).Error; err != nil {
		tx.Rollback()
		return err
	}

	// 然后删除标签记录
	if err := tx.Delete(&models.Tag{}, "id = ?", tagID).Error; err != nil {
		tx.Rollback()
		return err
	}

	return tx.Commit().Error
}

// DeleteFileTagRelation 删除文件和标签之间的关联关系
func DeleteFileTagRelation(fileID int64, tagID int64) error {
	return models.DB.Where("file_id = ? AND tag_id = ?", fileID, tagID).Delete(&models.FileTag{}).Error
}
