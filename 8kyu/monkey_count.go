package kata

func monkeyCount(n int) []int {
	monkeys := make([]int, 0, n)
	for i := 1; i <= n; i++ {
		monkeys = append(monkeys, i)
	}
	return monkeys
}
