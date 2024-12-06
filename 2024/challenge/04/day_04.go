package challenge

import (
	"bufio"
	"os"
	"strconv"
)

type Day04 struct{}

func (Day04) ID() string {
	return "04"
}

func (Day04) Part1(file *os.File) string {

	total := 0
	matrix := make([][]byte, 0)
	reader := bufio.NewScanner(file)
	for reader.Scan() {
		matrix = append(matrix, reader.Bytes())
	}

	width := len(matrix)
	height := len(matrix[0])
	getAndCheckBounds := func(c byte, x int, y int) bool {
		if x >= 0 && x < width && y >= 0 && y < height {
			return matrix[y][x] == c
		}
		return false
	}

	for y := 0; y < height; y++ {
		for x := 0; x < width; x++ {

			// LEFT AND RIGHT
			if getAndCheckBounds('X', x-3, y) &&
				getAndCheckBounds('M', x-2, y) &&
				getAndCheckBounds('A', x-1, y) &&
				getAndCheckBounds('S', x, y) {
				total++
			}
			if getAndCheckBounds('S', x-3, y) &&
				getAndCheckBounds('A', x-2, y) &&
				getAndCheckBounds('M', x-1, y) &&
				getAndCheckBounds('X', x, y) {
				total++
			}

			// UP AND DOWN
			if getAndCheckBounds('X', x, y-3) &&
				getAndCheckBounds('M', x, y-2) &&
				getAndCheckBounds('A', x, y-1) &&
				getAndCheckBounds('S', x, y) {
				total++
			}
			if getAndCheckBounds('S', x, y-3) &&
				getAndCheckBounds('A', x, y-2) &&
				getAndCheckBounds('M', x, y-1) &&
				getAndCheckBounds('X', x, y) {
				total++
			}

			// BACKSLASH
			if getAndCheckBounds('X', x-3, y-3) &&
				getAndCheckBounds('M', x-2, y-2) &&
				getAndCheckBounds('A', x-1, y-1) &&
				getAndCheckBounds('S', x, y) {
				total++
			}
			if getAndCheckBounds('S', x-3, y-3) &&
				getAndCheckBounds('A', x-2, y-2) &&
				getAndCheckBounds('M', x-1, y-1) &&
				getAndCheckBounds('X', x, y) {
				total++
			}

			// FORWARD SLASH
			if getAndCheckBounds('X', x+3, y-3) &&
				getAndCheckBounds('M', x+2, y-2) &&
				getAndCheckBounds('A', x+1, y-1) &&
				getAndCheckBounds('S', x, y) {
				total++
			}
			if getAndCheckBounds('S', x+3, y-3) &&
				getAndCheckBounds('A', x+2, y-2) &&
				getAndCheckBounds('M', x+1, y-1) &&
				getAndCheckBounds('X', x, y) {
				total++
			}
		}
	}
	return strconv.Itoa(total)
}

func (Day04) Part2(file *os.File) string {
	total := 0
	return strconv.Itoa(total)
}
