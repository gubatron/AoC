package main

import (
	"bufio"
	"fmt"
	"io"
	"math"
	"os"
	"strconv"
)

func main() {
	reader, err := os.Open("2019/1/input.txt")
	if err != nil {
		panic(err.Error())
	}
	moduleMassArray, err := ReadInts(reader)
	totalFuelRequired := 0

	test(12, 2)
	test(14, 2)
	test(1969, 654)
	test(100756, 33583)

	for i, m := range moduleMassArray {
		fi := fuel(m)
		totalFuelRequired += fi
		fmt.Printf("%d. mass=%d, fuel required=%d, total fuel=%d\n", i, m, fi, totalFuelRequired)
	}
}

func fuel(mass int) int {
	return int(math.Floor(float64(mass/3))) - 2
}

func test(mass int, expectedFuel int) {
	f := fuel(mass)
	if f == expectedFuel {
		fmt.Printf("OK: For a mass of %d we got the expected %d\n", mass, f)
	} else {
		fmt.Printf("KO: For a mass of %d we did not get the expected %d (got %d)\n", mass, expectedFuel, f)
	}
}

func ReadInts(r io.Reader) ([]int, error) {
	scanner := bufio.NewScanner(r)
	scanner.Split(bufio.ScanWords)
	var result []int
	for scanner.Scan() {
		x, err := strconv.Atoi(scanner.Text())
		if err != nil {
			return result, err
		}
		result = append(result, x)
	}
	return result, scanner.Err()
}
