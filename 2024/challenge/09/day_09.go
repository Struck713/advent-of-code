package challenge

import (
	"io"
	"os"
	"strconv"
)

type Day09 struct{}

func (Day09) ID() string {
	return "09"
}

func (Day09) Part1(file *os.File) string {

	bytes, _ := io.ReadAll(file)
	line := string(bytes)

	list := make([]int, 0)
	for index, c := range line {
		digit := int(c - '0')
		if index%2 == 0 {
			for j := 0; j < digit; j++ {
				list = append(list, index/2)
			}
		} else {
			for j := 0; j < digit; j++ {
				list = append(list, -1)
			}
		}
	}

	right := len(list) - 1
	for left := 0; left < right; left++ {
		if list[left] == -1 {
			list[left] = list[right]
			list[right] = -1
			for list[right] == -1 {
				right--
			}
		}
	}

	total := int64(0)
	for i, digit := range list {
		if digit != -1 {
			total += int64(digit * i)
		}
	}

	return strconv.FormatInt(total, 10)
}

func (Day09) Part2(file *os.File) string {

	bytes, _ := io.ReadAll(file)
	line := string(bytes)

	list := make([]int, 0)
	for index, c := range line {
		digit := int(c - '0')
		if index%2 == 0 {
			for j := 0; j < digit; j++ {
				list = append(list, index/2)
			}
		} else {
			for j := 0; j < digit; j++ {
				list = append(list, -1)
			}
		}
	}

	right := len(list) - 1
	for 1 <= right {
		digit := list[right]
		if digit != -1 {
			offset := 0
			for right-offset > 0 && list[right-offset] == digit {
				offset++
			}

			for left := 0; left < right-offset; left++ {
				space := 0
				for list[left+space] == -1 {
					space++
				}
				if offset <= space {
					for offset > 0 {
						list[left] = digit
						list[right] = -1
						left++
						right--
						offset--
					}
					break
				}
			}
			right -= offset
		} else {
			right--
		}

	}

	total := int64(0)
	for i, digit := range list {
		if digit != -1 {
			total += int64(digit * i)
		}
	}

	return strconv.FormatInt(total, 10)
}
