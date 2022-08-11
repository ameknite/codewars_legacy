package kata

func RemoveChar(word string) string {
	return word[1 : len(word)-1]
}

// package kata

// func RemoveChar(word string) string {
//   var newWord = []rune(word)
//   return string(newWord[1:len(newWord) - 1])
// }
