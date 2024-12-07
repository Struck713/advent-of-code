package challenge

import (
	"bufio"
	"os"
	"strconv"
)

type Day06 struct{}

func (Day06) ID() string {
	return "06"
}

func (Day06) Part1(file *os.File) string {

	total := 0

	matrix := [][]rune{}
	reader := bufio.NewScanner(file)
	for reader.Scan() {
		matrix = append(matrix, []rune(reader.Text()))
	}

	width, height := len(matrix), len(matrix[0])
	x, y := findStart(matrix)

	getCharacter := func(x int, y int) rune {
		if x >= 0 && x < width && y >= 0 && y < height {
			return matrix[y][x]
		}
		return ' '
	}

	dir := 0
	for x >= 0 && x < width && y >= 0 && y < height {

		nextX, nextY := 0, 0

		switch dir {
		case 0:
			nextY--
			break
		case 1:
			nextX++
			break
		case 2:
			nextY++
			break
		case 3:
			nextX--
			break
		}

		if getCharacter(x+nextX, y+nextY) == '#' {
			dir = (dir + 1) % 4
		} else {
			matrix[y][x] = 'V'
			x += nextX
			y += nextY
		}
	}

	for _, row := range matrix {
		for _, r := range row {
			if r == 'V' {
				total++
			}
		}
	}

	return strconv.Itoa(total)
}

func (Day06) Part2(file *os.File) string {
	total := 0
	return strconv.Itoa(total)
}

func findStart(matrix [][]rune) (int, int) {
	for y, row := range matrix {
		for x, r := range row {
			if r == '^' {
				return x, y
			}
		}
	}
	return 0, 0
}
