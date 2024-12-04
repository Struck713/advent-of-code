package challenge

import (
	"bufio"
	"os"
	"strconv"
	"strings"
)

type Day02 struct{}

func (Day02) ID() string {
	return "02"
}

func abs(a int64) int64 {
	if a >= 0 {
		return a
	}
	return a * -1
}

func isIncreasing(vals []int64) int {
	for i := 0; i < len(vals)-1; i++ {
		if vals[i+1] <= vals[i] {
			return i
		}
	}

	return -1
}

func isDecreasing(vals []int64) int {
	for i := 0; i < len(vals)-1; i++ {
		if vals[i+1] >= vals[i] {
			return i
		}
	}

	return -1
}

func isSafe(vals []int64) int {
	for i := 0; i < len(vals)-1; i++ {
		diff := abs(vals[i+1] - vals[i])
		if diff < 1 || diff > 3 {
			return i
		}
	}

	return -1
}

func (Day02) Part1(file *os.File) string {

	total := 0
	reader := bufio.NewScanner(file)
	for reader.Scan() {
		items := make([]int64, 0)
		for _, v := range strings.Split(reader.Text(), " ") {
			num, _ := strconv.ParseInt(v, 10, 64)
			items = append(items, num)
		}

		safe := isSafe(items)
		increase := isIncreasing(items)
		decrease := isDecreasing(items)

		if max(safe, min(increase, decrease)) == -1 {
			total++
		}

	}

	return strconv.Itoa(total)
}

func (Day02) Part2(file *os.File) string {

	total := 0
	reader := bufio.NewScanner(file)
	for reader.Scan() {
		items := make([]int64, 0)
		for _, v := range strings.Split(reader.Text(), " ") {
			num, _ := strconv.ParseInt(v, 10, 64)
			items = append(items, num)
		}

		safe := isSafe(items)
		increase := isIncreasing(items)
		decrease := isDecreasing(items)
		if max(safe, min(increase, decrease)) == -1 {
			total++
		}

	}

	return strconv.Itoa(total)
}
