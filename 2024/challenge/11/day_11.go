package challenge

import (
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"
)

type Day11 struct{}

func (Day11) ID() string {
	return "11"
}

func digits(item int) int {
	return len(strconv.Itoa(item))
}

func splitNumber(item int) (int, int) {
	str := strconv.Itoa(item)
	half := len(str) / 2
	left, _ := strconv.Atoi(str[:half])
	right, _ := strconv.Atoi(str[half:])
	return left, right
}

func solveSubtree(cache *map[int]int, item int, n int) int {
	if n == 0 {
		return 1
	}

	if item == 0 {
		_, exists := (*cache)[n-1]
		if exists {
			return (*cache)[n-1]
		}
		(*cache)[n-1] = solveSubtree(cache, 1, n-1)
		return (*cache)[n-1]
	} else if digits(item)%2 == 0 {
		left, right := splitNumber(item)
		return solveSubtree(cache, left, n-1) + solveSubtree(cache, right, n-1)
	}
	return solveSubtree(cache, item*2024, n-1)
}

func solve(file *os.File, reps int) int {

	bytes, _ := io.ReadAll(file)
	data := string(bytes)

	items := []int{}
	for _, v := range strings.Split(data, " ") {
		num, _ := strconv.Atoi(v)
		items = append(items, num)
	}

	cache := map[int]int{}

	score := 0
	for _, item := range items {
		score += solveSubtree(&cache, item, reps)
	}

	fmt.Println(cache)
	return score
}

func (Day11) Part1(file *os.File) string {
	return strconv.Itoa(solve(file, 25))
}

func (Day11) Part2(file *os.File) string {
	return strconv.Itoa(solve(file, 75))
}
