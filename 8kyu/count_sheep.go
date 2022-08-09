package kata

import (
	"fmt"
)

func countSheep(num int) string {
	var res string
	for i := 1; i <= num; i++ {
		res += fmt.Sprintf("%v sheep...", i)
	}
	return res
}


// package kata

// import (
//   "fmt"
//   "strings"
// )

// func countSheep(num int) string {
//   var sb strings.Builder

//   for count := 1; count <= num; count++ {
//         fmt.Fprintf(&sb, "%d sheep...", count)
//   }

//   return sb.String()
// }
