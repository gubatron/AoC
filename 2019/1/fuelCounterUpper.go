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

	testFuel(5, 0)
	testFuel(12, 2)
	testFuel(14, 2)
	testFuel(1969, 654)
	testFuel(100756, 33583)
	fmt.Printf("=============================================================================\n")
	testFuelRecursive(14, 2)
	testFuelRecursive(1969, 966)
	testFuelRecursive(100756, 50346)
	fmt.Printf("=============================================================================\n")

	for i, m := range moduleMassArray {
		fi := fuel(m)
		totalFuelRequired += fi
		fmt.Printf("%d. mass=%d, fuel required=%d, total fuel=%d\n", i, m, fi, totalFuelRequired)
	}

	fmt.Printf("=============================================================================\n")
	fmt.Printf("For the total fuel %d, we actually need -> %d\n", totalFuelRequired, fuelRecursive(totalFuelRequired))
	fmt.Printf("=============================================================================\n")

	totalFuelRequired = 0
	for i, m := range moduleMassArray {
		fi := fuelRecursive(m)
		totalFuelRequired += fi
		fmt.Printf("%d. mass=%d, fuel required=%d, total fuel=%d\n", i, m, fi, totalFuelRequired)
	}

}

func fuel(mass int) int {
	f := int(math.Floor(float64(mass/3))) - 2
	if f > 0 {
		return f
	}
	return 0
}

func fuelRecursive(mass int) int {
	if fuel(mass) > 0 {
		return fuel(mass) + fuelRecursive(fuel(mass))
	}
	return 0
}

func testFuel(mass int, expectedFuel int) {
	f := fuel(mass)
	if f == expectedFuel {
		fmt.Printf("OK: For a mass of %d we got the expected %d\n", mass, f)
	} else {
		fmt.Printf("KO: For a mass of %d we did not get the expected %d (got %d)\n", mass, expectedFuel, f)
	}
}

func testFuelRecursive(mass int, expectedFF int) {
	f := fuelRecursive(mass)
	if f == expectedFF {
		fmt.Printf("OK: For a mass of %d we got the expected recursive %d\n", mass, f)
	} else {
		fmt.Printf("KO: For a mass of %d we did not get the expected recursive %d (got %d)\n", mass, expectedFF, f)
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
