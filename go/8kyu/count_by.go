package kata

func CountBy(x, n int) []int {
	res := make([]int, 0, n)
	for i := x; i <= n*x; i += x {
		res = append(res, i)
	}
	return res
}
