package utils

import (
	"strconv"
)

// StringToInt64 将字符串转换为 int64 类型
func StringToInt64(s string) (int64, error) {
	i, err := strconv.ParseInt(s, 10, 64)
	if err != nil {
		return 0, err
	}
	return i, nil
}
