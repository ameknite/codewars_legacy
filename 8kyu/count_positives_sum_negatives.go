package kata

func CountPositivesSumNegatives(numbers []int) []int {
	res := make([]int, 0, cap(numbers))
	var count, sum int
	for _, v := range numbers {
		if v > 0 {
			count++
		} else {
			sum += v
		}
	}

	res = append(res, count)
	res = append(res, sum)
	return res
}
