package kata

func Maps(x []int) []int {
	y := make([]int, 0, len(x))
	for _, v := range x {
		y = append(y, v*2)
	}
	return y
}
