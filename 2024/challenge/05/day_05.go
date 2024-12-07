package challenge

import (
	"bufio"
	"os"
	"strconv"
	"strings"
)

type Day05 struct{}

func (Day05) ID() string {
	return "05"
}

func (Day05) Part1(file *os.File) string {
	total := 0

	rules := map[int64][]int64{}
	reader := bufio.NewScanner(file)
	for reader.Scan() {
		text := reader.Text()
		if text == "" {
			break
		}

		rule := strings.Split(text, "|")
		page, _ := strconv.ParseInt(rule[0], 10, 64)
		before, _ := strconv.ParseInt(rule[1], 10, 64)

		list, exists := rules[page]
		if !exists {
			list = []int64{}
		}

		rules[page] = append(list, before)
	}

	for reader.Scan() {
		list := make([]int64, 0)
		for _, item := range strings.Split(reader.Text(), ",") {
			number, _ := strconv.ParseInt(item, 10, 64)
			list = append(list, number)
		}

		safe := true
		for i, number := range list {
			rules, exists := rules[number]
			if exists {
				for _, rule := range rules {
					for j := i; j >= 0; j-- {
						if rule == list[j] {
							safe = false
							break
						}
					}
				}
			}
		}

		if safe {
			total += int(list[len(list)/2])
		}

	}

	return strconv.Itoa(total)
}

func (Day05) Part2(file *os.File) string {
	total := 0

	rules := map[int64][]int64{}
	reader := bufio.NewScanner(file)
	for reader.Scan() {
		text := reader.Text()
		if text == "" {
			break
		}

		rule := strings.Split(text, "|")
		page, _ := strconv.ParseInt(rule[0], 10, 64)
		before, _ := strconv.ParseInt(rule[1], 10, 64)

		list, exists := rules[page]
		if !exists {
			list = []int64{}
		}

		rules[page] = append(list, before)
	}

	for reader.Scan() {
		list := make([]int64, 0)
		for _, item := range strings.Split(reader.Text(), ",") {
			number, _ := strconv.ParseInt(item, 10, 64)
			list = append(list, number)
		}

		safe := true
		for i := 0; i < len(list); i++ {
			rules, exists := rules[list[i]]
			if exists {
				for _, rule := range rules {
					for j := 0; j <= i; j++ {
						if rule == list[j] {
							safe = false
							temp := list[i]
							list[i] = list[j]
							list[j] = temp
							i = 0
						}
					}
				}
			}
		}

		if !safe {
			total += int(list[len(list)/2])
		}

	}

	return strconv.Itoa(total)
}
