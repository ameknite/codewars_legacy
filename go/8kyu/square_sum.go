package kata

func SquareSum(numbers []int) int {
	var sum int
	for _, v := range numbers {
		sum+= v * v
	}
	return sum
}
