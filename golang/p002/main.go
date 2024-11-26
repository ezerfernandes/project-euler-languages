package main

import "fmt"

func main() {
	var sum int
	prev, curr := 1, 2
	for curr < 4_000_000 {
		if curr%2 == 0 {
			sum += curr
		}

		prev, curr = curr, prev+curr
	}

	fmt.Println(sum)
}
