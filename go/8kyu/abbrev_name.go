package kata

import (
	"strings"
	"unicode"
)

func AbbrevName(name string) string {
	var fields []string
	for _, v := range strings.Fields(name) {
		runes := []rune(v)
		fields = append(fields, string(unicode.ToUpper(runes[0])))
	}
	return strings.Join(fields, ".")
}
