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
	matrix := make([][]rune, 0)
	reader := bufio.NewScanner(file)
	for reader.Scan() {
		matrix = append(matrix, []rune(reader.Text()))
	}

	width := len(matrix)
	height := len(matrix[0])

	getAndCheckBounds := func(c rune, x int, y int) bool {
		return x >= 0 && x < width && y >= 0 && y < height && matrix[x][y] == c
	}

	for y := 0; y < height; y++ {
		for x := 0; x < width; x++ {

			// LEFT AND RIGHT
			if getAndCheckBounds('X', x, y) &&
				getAndCheckBounds('M', x+1, y) &&
				getAndCheckBounds('A', x+2, y) &&
				getAndCheckBounds('S', x+3, y) {
				total++
			}

			if getAndCheckBounds('S', x, y) &&
				getAndCheckBounds('A', x+1, y) &&
				getAndCheckBounds('M', x+2, y) &&
				getAndCheckBounds('X', x+3, y) {
				total++
			}

			// UP AND DOWN
			if getAndCheckBounds('X', x, y) &&
				getAndCheckBounds('M', x, y+1) &&
				getAndCheckBounds('A', x, y+2) &&
				getAndCheckBounds('S', x, y+3) {
				total++
			}
			if getAndCheckBounds('S', x, y) &&
				getAndCheckBounds('A', x, y+1) &&
				getAndCheckBounds('M', x, y+2) &&
				getAndCheckBounds('X', x, y+3) {
				total++
			}

			// BACKSLASH
			if getAndCheckBounds('X', x, y) &&
				getAndCheckBounds('M', x+1, y+1) &&
				getAndCheckBounds('A', x+2, y+2) &&
				getAndCheckBounds('S', x+3, y+3) {
				total++
			}
			if getAndCheckBounds('S', x, y) &&
				getAndCheckBounds('A', x+1, y+1) &&
				getAndCheckBounds('M', x+2, y+2) &&
				getAndCheckBounds('X', x+3, y+3) {
				total++
			}

			// FORWARD SLASH
			if getAndCheckBounds('X', x, y) &&
				getAndCheckBounds('M', x+1, y-1) &&
				getAndCheckBounds('A', x+2, y-2) &&
				getAndCheckBounds('S', x+3, y-3) {
				total++
			}
			if getAndCheckBounds('S', x, y) &&
				getAndCheckBounds('A', x+1, y-1) &&
				getAndCheckBounds('M', x+2, y-2) &&
				getAndCheckBounds('X', x+3, y-3) {
				total++
			}
		}
	}
	return strconv.Itoa(total)
}

func (Day04) Part2(file *os.File) string {

	total := 0
	matrix := make([][]rune, 0)
	reader := bufio.NewScanner(file)
	for reader.Scan() {
		matrix = append(matrix, []rune(reader.Text()))
	}

	width := len(matrix)
	height := len(matrix[0])

	getCharacter := func(x int, y int) rune {
		if x >= 0 && x < width && y >= 0 && y < height {
			return matrix[x][y];
		}
		return ' '
	}

	for y := 0; y < height; y++ {
		for x := 0; x < width; x++ {
			if getCharacter(x, y) == 'A' {
				pattern := string([]rune{
					getCharacter(x - 1, y + 1),
					getCharacter(x + 1, y + 1),
					getCharacter(x + 1, y - 1),
					getCharacter(x - 1, y - 1),
				})

				if pattern == "MSSM" || pattern == "MMSS" || pattern == "SSMM" || pattern == "SMMS" {
					total++
				}

			}
		}
	}
	return strconv.Itoa(total)
}
