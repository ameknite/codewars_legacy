package kata

func Points(games []string) int {
	var total int
	for _, v := range games {
		if v[0] > v[2] {
			total += 3
		} else if v[0] == v[2] {
			total++
		}
	}
	return total
}
