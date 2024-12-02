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

func (Day02) Part1(file *os.File) string {

	total := 0
	reader := bufio.NewScanner(file)
	for reader.Scan() {
		items := make([]int64, 0)
		for _, v := range strings.Split(reader.Text(), " ") {
			num, _ := strconv.ParseInt(v, 10, 64)
			items = append(items, num)
		}

		safe := true
		increasing := false
		decreasing := false

		for i := 0; i < len(items)-1; i++ {
			diff := items[i+1] - items[i]
			if diff >= 0 {
				increasing = true
			} else {
				decreasing = true
			}

			if increasing && decreasing {
				safe = false
				break
			}

			diffAbs := abs(diff)
			if diffAbs < 1 || diffAbs > 3 {
				safe = false
				break
			}
		}

		if safe {
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

		length := len(items)
		increasing := false
		decreasing := false

		for i := 0; i < len(items)-1; i++ {
			diff := items[i+1] - items[i]
			if diff >= 0 {
				increasing = true
			} else {
				decreasing = true
			}

			if increasing && decreasing {
				items = append(items[:i], items[i+1:]...)
				i = 0
				increasing = false
				decreasing = false
				continue
			}

			diffAbs := abs(diff)
			if diffAbs < 1 || diffAbs > 3 {
				items = append(items[:i], items[i+1:]...)
				i = 0
				continue
			}
		}

		if len(items) + 1 < length {
			total++
		}

	}

	return strconv.Itoa(total)
}
