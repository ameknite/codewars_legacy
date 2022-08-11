package kata

func BoolToWord(word bool) string {
	switch word {
	case true:
		return "Yes"
	default:
		return "No"
	}
}
