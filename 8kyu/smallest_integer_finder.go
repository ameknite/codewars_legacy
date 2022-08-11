package kata

func SmallestIntegerFinder(numbers []int) int {
	min := numbers[0]
	for _, v := range numbers[1:] {
		if v < min {
			min = v
		}
	}
	return min
}
