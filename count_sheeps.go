package kata

func CountSheeps(numbers []bool) int {
	var total int
	for _, v := range numbers {
		if v {
			total++
		}
	}
	return total
}
