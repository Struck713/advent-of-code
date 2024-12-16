package challenge

import (
	"bufio"
	"math"
	"os"
	"strconv"
	"strings"
)

type Day07 struct{}

func (Day07) ID() string {
	return "07"
}

func IsPossibility(items []int64, answer int64) bool {
	for i := 0.0; i < math.Pow(2, float64(len(items)-1)); i++ {
		step := uint(i)
		result := items[0]
		for _, item := range items[1:] {
			if (step & 0x1) == 1 {
				result *= item
			} else {
				result += item
			}
			step = (step >> 1)
		}

		if result == answer {
			return true
		}
	}
	return false
}

func (Day07) Part1(file *os.File) string {

	total := int64(0)

	// data := map[int][]int{}
	reader := bufio.NewScanner(file)
	for reader.Scan() {
		row := strings.Split(reader.Text(), ": ")
		answer, _ := strconv.ParseInt(row[0], 10, 64)

		items := []int64{}
		for _, item := range strings.Split(row[1], " ") {
			num, _ := strconv.ParseInt(item, 10, 64)
			items = append(items, num)
		}

		if IsPossibility(items, answer) {
			total += answer
		}

	}

	return strconv.FormatInt(total, 10)
}

func AppendInt(left int64, right int64) int64 {
	return left*int64(math.Pow10(len(strconv.FormatInt(right, 10)))) + right
}

func Calculate(items []int64, answer int64) bool {
	if len(items) == 1 {
		return items[0] == answer
	}

	return Calculate(append([]int64{items[0] + items[1]}, items[2:]...), answer) ||
		Calculate(append([]int64{items[0] * items[1]}, items[2:]...), answer) ||
		Calculate(append([]int64{AppendInt(items[0], items[1])}, items[2:]...), answer)
}

func (Day07) Part2(file *os.File) string {
	total := int64(0)

	reader := bufio.NewScanner(file)
	for reader.Scan() {
		row := strings.Split(reader.Text(), ": ")
		answer, _ := strconv.ParseInt(row[0], 10, 64)

		items := []int64{}
		for _, item := range strings.Split(row[1], " ") {
			num, _ := strconv.ParseInt(item, 10, 64)
			items = append(items, num)
		}

		if Calculate(items, answer) {
			total += answer
		}
	}

	return strconv.FormatInt(total, 10)
}
