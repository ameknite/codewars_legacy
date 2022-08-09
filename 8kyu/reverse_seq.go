package kata

func ReverseSeq(n int) []int {
	seq := make([]int, 0, n)
	for i := n; i > 0; i-- {
		seq = append(seq, i)
	}
	return seq
}
