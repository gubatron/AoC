package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
)

type Asteroid struct {
	x      int
	y      int
	angles map[float64]*Asteroid
}

func main() {
	detectionTestN(1, 3,4, 8)
	detectionTestN(2, 5, 8, 33)
	detectionTestN(3, 1,2, 35)
	detectionTestN(4, 6, 3, 41)
	detectionTestN(5, 11,13, 210)
	part1()
	part2()
}

func detectionTestN(n int, expected_x int, expected_y int, other_asteroids int) {
	asteroids := loadAsteroids(fmt.Sprintf("2019/10/test_%d.txt", n))
	winner := pickBestAsteroid(asteroids)
	fmt.Printf("Test %d: %d, %d => %d asteroids     Expected: %d, %d => %d asteroids\n", n, winner.x, winner.y, winner.visibleAsteroids(), expected_x, expected_y, other_asteroids)
}

func part1() {
	asteroids := loadAsteroids("2019/10/input.txt")
	winner := pickBestAsteroid(asteroids) //22,19 -> 282 asteroids
	fmt.Printf("Part 1: %d, %d => %d asteroids\n", winner.x, winner.y, winner.visibleAsteroids())
}

func part2() {

}

func pickBestAsteroid(asteroids []*Asteroid) *Asteroid {
	maxVisibleAsteroids := 0
	var winner *Asteroid

	for _, asteroid := range asteroids {
		for _, other := range asteroids {
			if asteroid.x == other.x && asteroid.y == other.y {
				continue
			}
			asteroid.angleTo(other)
		}
		visibleAsteroids := asteroid.visibleAsteroids()
		if visibleAsteroids > maxVisibleAsteroids {
			winner = asteroid
			maxVisibleAsteroids = visibleAsteroids
		}
	}
	return winner
}

func loadAsteroids(filePath string) []*Asteroid {
	file, err := os.Open(filePath)
	if err != nil {
		panic(err.Error())
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)

	y := 0

	asteroids := []*Asteroid{}

	for scanner.Scan() {
		line := scanner.Text()
		for x, character := range line {
			if character == '#' {
				asteroids = append(asteroids, &Asteroid{x, y, make(map[float64]*Asteroid)})
			}
		}
		y++
	}

	return asteroids
}

func (this *Asteroid) angleTo(other *Asteroid) float64 {
	angle := math.Atan2(float64(other.y-this.y), float64(other.x-this.x))

	if this.angles == nil {
		panic("check your logic, an asteroid needs it's angles map initialized!")
	}

	if this.angles[angle] == nil {
		this.angles[angle] = other
	} else {
		previous := this.angles[angle]
		if this.manhattanDistance(other) < this.manhattanDistance(previous) {
			this.angles[angle] = other
		}
	}

	return angle
}

func (this *Asteroid) printVisibleAsteroids() {
	i := 1
	for slope, asteroid := range this.angles {
		fmt.Printf("%d slope=%f to %d,%d\n", 1+i, slope, asteroid.x, asteroid.y)
		i++
	}
}

func (this *Asteroid) visibleAsteroids() int {
	if this.angles == nil {
		return 0
	}
	return len(this.angles)
}

func (this *Asteroid) manhattanDistance(other *Asteroid) int {
	return int(math.Abs(float64(other.x-this.x)) + math.Abs(float64(other.y-this.y)))
}

func GCD(a, b int) int {
	for b != 0 {
		t := b
		b = a % b
		a = t
	}
	return a
}
