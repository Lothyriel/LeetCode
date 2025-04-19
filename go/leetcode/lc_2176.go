package main

import "fmt"

func main() {
	result := countPairs([]int{3, 1, 2, 2, 2, 1, 3}, 2)
	fmt.Println("Result is", result)
}

func countPairs(nums []int, k int) int {
	pairs := 0

	for i, val := range nums {
		for j := i + 1; j < len(nums); j++ {
			if i == j {
				continue
			}

			divisible := (i*j)%k == 0

			other := nums[j]

			if divisible && val == other {
				pairs += 1
			}
		}
	}

	return pairs
}
