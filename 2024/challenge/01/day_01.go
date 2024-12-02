package challenge

import (
	"fmt"
	"io"
	"os"
	"sort"
	"strconv"
)

type Day01 struct{}

func (Day01) ID() string {
	return "01"
}

func (Day01) LoadValues(file *os.File) ([]uint64, []uint64) {
	left := make([]uint64, 5)
	right := make([]uint64, 5)

	for {
		var a, b uint64
		_, err := fmt.Fscanf(file, "%d %d", &a, &b)
		if err != nil {
			if err == io.EOF {
				break
			}
			panic("Failed to read file.")
		}
		left = append(left, a)
		right = append(right, b)
	}

	sort.Slice(left, func(i, j int) bool {
		return left[i] < left[j]
	})

	sort.Slice(right, func(i, j int) bool {
		return right[i] < right[j]
	})

	return left, right
}

func (d Day01) Part1(file *os.File) string {

	left, right := d.LoadValues(file)

	total := uint64(0)
	for index := range left {
		a := left[index]
		b := right[index]
		sub := max(a, b) - min(a, b)
		total += sub
	}

	return strconv.FormatUint(total, 10)
}

func (d Day01) Part2(file *os.File) string {

	left, right := d.LoadValues(file)

	total := uint64(0)
	for index := range left {
		a := left[index]
		similarity := uint64(0)
		for _, b := range right {
			if a == b {
				similarity += 1
			}
		}
		total += (similarity * a)
	}

	return strconv.FormatUint(total, 10)
}
