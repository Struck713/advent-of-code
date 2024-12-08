package challenge

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

type Day06 struct{}

func (Day06) ID() string {
	return "06"
}

func (Day06) Part1(file *os.File) string {

	total := 0
	matrix, width, height, x, y := buildMatrix(file)
	fmt.Println(x, y)

	inbounds := func(x int, y int) bool {
		return x >= 0 && x < width && y >= 0 && y < height
	}
	getCharacter := func(x int, y int) rune {
		if inbounds(x, y) {
			return matrix[y][x]
		}
		return ' '
	}

	dir := 0
	for inbounds(x, y) {

		nextX, nextY := x, y

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

		if getCharacter(nextX, nextY) == '#' {
			dir = (dir + 1) % 4
		} else {
			if getCharacter(x, y) != 'V' {
				total++
				matrix[y][x] = 'V'
			}
			x = nextX
			y = nextY
		}
	}

	return strconv.Itoa(total)
}

func (Day06) Part2(file *os.File) string {
	total := 0
	matrix, width, height, x, y := buildMatrix(file)
	inbounds := func(x int, y int) bool {
		return x >= 0 && x < width && y >= 0 && y < height
	}
	getCharacter := func(x int, y int) rune {
		if inbounds(x, y) {
			return matrix[y][x]
		}
		return ' '
	}

	obstacles := make([][]int, 0)

	dir := 0
	for inbounds(x, y) {

		nextX, nextY := x, y

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

		nextDir := (dir + 1) % 4
		next := getCharacter(nextX, nextY)
		if next == '#' {
			dir = nextDir
			obstacles = append([][]int{[]int{nextX, nextY}}, obstacles...)
		} else {
			if !(next == '*' || next == ' ') {
				switch nextDir {
				case 0:
					for _, obstacle := range obstacles {
						if x == obstacle[0] && y > obstacle[1] {
							matrix[nextY][nextX] = '*'
							total++
							break
						}
					}
					break
				case 1:
					for _, obstacle := range obstacles {
						if x < obstacle[0] && y == obstacle[1] {
							matrix[nextY][nextX] = '*'
							total++
							break
						}
					}
					break
				case 2:
					for _, obstacle := range obstacles {
						if x == obstacle[0] && y < obstacle[1] {
							matrix[nextY][nextX] = '*'
							total++
							break
						}
					}
					break
				case 3:
					for _, obstacle := range obstacles {
						if x > obstacle[0] && y == obstacle[1] {
							matrix[nextY][nextX] = '*'
							total++
							break
						}
					}
					break
				}
			}
			x = nextX
			y = nextY
		}
	}

	// print(matrix)

	return strconv.Itoa(total)
}

func buildMatrix(file *os.File) ([][]rune, int, int, int, int) {
	matrix := [][]rune{}
	reader := bufio.NewScanner(file)
	for reader.Scan() {
		matrix = append(matrix, []rune(reader.Text()))
	}

	x, y := findStart(matrix)
	return matrix, len(matrix), len(matrix[0]), x, y
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
