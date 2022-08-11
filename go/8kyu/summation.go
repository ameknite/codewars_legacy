package kata

func Summation(n int) int {
	var total int
	for i := 1; i <= n; i++ {
		total += i
	}
	return total
}
