package challenge

import (
	"bufio"
	"os"
	"strconv"
	"strings"
)

type Day07 struct{}

func (Day07) ID() string {
	return "07"
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

		results := (len(items) - 1) * 2

		for i := 0; i < results; i++ {
			step := uint(i)
			result := items[0]
			for _, item := range items[1:] {
				// fmt.Printf("%d => %b %d\n", step, step, step>>1)
				// fmt.Println(step, step&1 == 1)
				if (step & 1) == 1 {
					result *= item
				} else {
					result += item
				}
				step = (step >> 1)
			}

			if result == answer {
				total += result
				break
			}
		}

	}

	return strconv.FormatInt(total, 10)
}

func (Day07) Part2(file *os.File) string {
	total := 0
	return strconv.Itoa(total)
}
