package challenge

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

type Day10 struct{}

func (Day10) ID() string {
	return "10"
}

func inbounds(matrix [][]int, x int, y int) bool {
	return x >= 0 && x < len(matrix[0]) && y >= 0 && y < len(matrix)
}

func scoreTrail(matrix [][]int, x int, y int, next int) int {
	if !inbounds(matrix, x, y) {
		return 0
	}

	digit := matrix[y][x]
	if next != digit {
		return 0
	}
	
	if digit == 9 {
		return 1
	}

	return scoreTrail(matrix, x+1, y, next+1) +
		scoreTrail(matrix, x-1, y, next+1) +
		scoreTrail(matrix, x, y+1, next+1) +
		scoreTrail(matrix, x, y-1, next+1)

}

func (Day10) Part1(file *os.File) string {
	total := 0

	matrix := [][]int{}
	reader := bufio.NewScanner(file)
	for reader.Scan() {
		items := []int{}
		for _, r := range reader.Text() {
			items = append(items, int(r-'0'))
		}
		matrix = append(matrix, items)
	}

	total = 0
	for y, row := range matrix {
		for x, value := range row {
			if value == 0 {
				score := scoreTrail(matrix, x, y, 0)
				total += score
				fmt.Println(score)
			}
		}
	}

	return strconv.Itoa(total)
}

func (Day10) Part2(file *os.File) string {
	total := int64(0)
	return strconv.FormatInt(total, 10)
}
