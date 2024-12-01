package day01

import (
	"fmt"
	"io"
	"os"
	"sort"
)

func Run() {
	file, err := os.Open("01/chal.input")
	if err != nil {
		panic("Failed to open challenge input file.")
	}

	left := make([]uint64, 5)
	right := make([]uint64, 5)

	for {
		var a, b uint64
		_, err := fmt.Fscanf(file, "%d %d", &a, &b)
		if err != nil {
			if err == io.EOF {
				break
			}
			fmt.Println("Error reading file:", err)
			return
		}
		left = append(left, a)
		right = append(right, b)
	}

	Part1(left, right)
	Part2(left, right)
}

func Part1(left []uint64, right []uint64) {
	sort.Slice(left, func(i, j int) bool {
		return left[i] < left[j]
	})

	sort.Slice(right, func(i, j int) bool {
		return right[i] < right[j]
	})

	total := uint64(0)
	for index := range left {
		a := left[index]
		b := right[index]
		sub := max(a, b) - min(a, b)
		total += sub
	}

	fmt.Println(total)
}

func Part2(left []uint64, right []uint64) {
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

	fmt.Println(total)
}
