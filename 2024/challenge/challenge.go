package challenge

import (
	"fmt"
	"os"
)

type Challenge interface {
	Part1(*os.File) string
	Part2(*os.File) string
	ID() string
}

func Run(c Challenge) {

	id := c.ID()
	file, err := os.Open("challenge/" + id + "/chal.input")
	if err != nil {
		panic("Failed to open challenge input file.")
	}
	fmt.Println(id + " => Part 1: " + c.Part1(file))

	file.Seek(0, 0)
	fmt.Println(id + " => Part 2: " + c.Part2(file))

	file.Close()
}
