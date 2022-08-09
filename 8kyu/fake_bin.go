package kata

func FakeBin(x string) string {
	runes := []rune(x)
	for i, v := range runes {
		if v < '5' {
			runes[i] = '0'
		} else {
			runes[i] = '1'
		}
	}
	return string(runes)
}
