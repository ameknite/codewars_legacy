package kata

import "strings"

func ReverseWords(str string) string {
	seq := strings.Fields(str)
	for i, j := 0, len(seq) - 1; i < j; i, j = i+1, j-1 {
		seq[i], seq[j] = seq[j], seq[i]
	}
	return strings.Join(seq, " ")
}
