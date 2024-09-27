package typings

type CreateTagRequest struct {
	Name *string `json:"tag_name"`
}
type CreateFileRequest struct {
	Name *string `json:"file_name"`
	Type *string `json:"file_type"`
}

type CreateFileTagRelationRequest struct {
	FileID *int64 `json:"file_id"`
	TagID  *int64 `json:"tag_id"`
}

type FindFileRequest struct {
	ID         *int64  `json:"file_id"`
	Type       *string `json:"file_type"`
	Location   *string `json:"file_location"`
	Name       *string `json:"file_name"`
	ClickCount *int64  `json:"file_click_count"`
	ShareCount *int64  `json:"file_share_count"`
}
