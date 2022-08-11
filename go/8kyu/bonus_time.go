package kata

import (
	"fmt"
	"strings"
)

func BonusTime(salary int, bonus bool) string {
	var sb strings.Builder
	sb.WriteString("£")
	if bonus {
		fmt.Fprintf(&sb, "%v", salary*10)
	} else {
		fmt.Fprintf(&sb, "%v", salary)
	}
	return sb.String()
}
