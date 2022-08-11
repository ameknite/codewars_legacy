package kata

func Grow(arr []int) int {
	total := 1
	for _, v := range arr {
		total *= v
	}
	return total
}
