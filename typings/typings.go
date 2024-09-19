package typings

type Tag struct {
	Name *string `json:"name"`
	ID   *int64  `json:"id"`
}
type File struct {
	Name *string `json:"name"`
	ID   *int64  `json:"id"`
	Type *string `json:"type"`
}

type FileTag struct {
	FileID *int64 `json:"file_id"`
	TagID  *int64 `json:"tag_id"`
}
