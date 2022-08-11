package kata

func PositiveSum(numbers []int) int {
	var total int
	for _, v := range numbers {
		if v > 0 {
			total += v
		}
	}
	return total
}

