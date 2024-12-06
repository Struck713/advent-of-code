package challenge

import (
	"bufio"
	"io"
	"os"
	"regexp"
	"strconv"
)

type Day03 struct{}

func (Day03) ID() string {
	return "03"
}

func (Day03) Part1(file *os.File) string {

	total := 0
	reader := bufio.NewReader(file)
	bytes, _ := io.ReadAll(reader)
	data := string(bytes)

	reg := regexp.MustCompile(`mul\((\d+),(\d+)\)`)
	for _, match := range reg.FindAllStringSubmatch(data, -1) {
		left, _ := strconv.Atoi(match[1])
		right, _ := strconv.Atoi(match[2])
		total += (left * right)
	}

	return strconv.Itoa(total)
}

func (Day03) Part2(file *os.File) string {

	total := 0
	reader := bufio.NewReader(file)
	bytes, _ := io.ReadAll(reader)
	data := string(bytes)

	reg := regexp.MustCompile(`mul\((\d+),(\d+)\)`)
	dosAndDontsReg := regexp.MustCompile(`do\(\)|don't\(\)`)
	dosAndDonts := dosAndDontsReg.FindAllStringIndex(data, -1)

	latest := 0
	do := true
	for _, pair := range reg.FindAllStringIndex(data, -1) {
		for latest < len(dosAndDonts) && dosAndDonts[latest][0] < pair[0] {
			do = (dosAndDonts[latest][1] - dosAndDonts[latest][0]) == 4
			latest++
		}

		if do {
			match := reg.FindStringSubmatch(data[pair[0]:pair[1]])
			left, _ := strconv.Atoi(match[1])
			right, _ := strconv.Atoi(match[2])
			total += (left * right)
		}
	}

	return strconv.Itoa(total)
}
