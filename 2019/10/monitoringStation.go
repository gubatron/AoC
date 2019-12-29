package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"sort"
)

type Asteroid struct {
	x      int
	y      int
	angles map[float64]*Asteroid
}

const RADIAN_TO_DEGREE = 180 / math.Pi

func main() {
	detectionTestN(1, 3, 4, 8)
	detectionTestN(2, 5, 8, 33)
	detectionTestN(3, 1, 2, 35)
	detectionTestN(4, 6, 3, 41)
	detectionTestN(5, 11, 13, 210)
	part1()
	part2()
}

func detectionTestN(n int, expected_x int, expected_y int, other_asteroids int) {
	asteroids := loadAsteroids(fmt.Sprintf("2019/10/test_%d.txt", n))
	winner := pickBestAsteroid(asteroids)
	fmt.Printf("Test %d: %d, %d => %d asteroids     Expected: %d, %d => %d asteroids\n", n, winner.x, winner.y, winner.numVisibleAsteroids(), expected_x, expected_y, other_asteroids)
}

func part1() {
	asteroids := loadAsteroids("2019/10/input.txt")
	winner := pickBestAsteroid(asteroids) //22,19 -> 282 asteroids
	fmt.Printf("Part 1: %d, %d => %d asteroids      Expected: 22, 19 => 282\n", winner.x, winner.y, winner.numVisibleAsteroids())
}

func part2() {
	asteroids := loadAsteroids("2019/10/input.txt")
	monitoringStation := pickBestAsteroid(asteroids)
	vaporized := make([]*Asteroid, 0)

	for len(asteroids) > 1 {
		vaporizeThese := monitoringStation.getAsteroidsToVaporize(asteroids)
		asteroids = removeAsteroids(vaporizeThese, asteroids)
		monitoringStation.clearNearestVisible()
		vaporized = append(vaporized, vaporizeThese...)
	}
	fmt.Printf("Part 2: The 200th asteroid to be vaporized is at %d,%d, the answer is: %d",
		vaporized[199].x, vaporized[199].y, (vaporized[199].x*100)+vaporized[199].y)
}

func (station *Asteroid) getAsteroidsToVaporize(asteroids []*Asteroid) []*Asteroid {
	if station.numVisibleAsteroids() == 0 {
		station.pickNearestAsteroidsFrom(asteroids)
		if station.numVisibleAsteroids() == 0 {
			return nil
		}
	}
	vaporizeThese := make([]*Asteroid, 0)
	for _, asteroid := range station.angles {
		vaporizeThese = append(vaporizeThese, asteroid)
	}
	sort.Slice(vaporizeThese, func(i int, j int) bool {
		return station.angleTo(vaporizeThese[i]) < station.angleTo(vaporizeThese[j])
	})
	return vaporizeThese
}

func (this *Asteroid) pickNearestAsteroidsFrom(asteroids []*Asteroid) {
	this.clearNearestVisible()
	for _, other := range asteroids {
		if this.x == other.x && this.y == other.y {
			continue
		}
		this.angleTo(other)
	}
}

func pickBestAsteroid(asteroids []*Asteroid) *Asteroid {
	maxVisibleAsteroids := 0
	var winner *Asteroid

	for _, asteroid := range asteroids {
		asteroid.pickNearestAsteroidsFrom(asteroids)
		visibleAsteroids := asteroid.numVisibleAsteroids()
		if visibleAsteroids > maxVisibleAsteroids {
			winner = asteroid
			maxVisibleAsteroids = visibleAsteroids
		}
	}
	return winner
}

func removeAsteroids(removeThese []*Asteroid, fromThese []*Asteroid) []*Asteroid {
	for _, removeMe := range removeThese {
		i := findAsteroid(removeMe, fromThese)
		if i != -1 {
			fromThese = removeAsteroidAt(i, fromThese)
		} else {
			fmt.Printf("Could not find asteroid %d,%d\n", removeMe.x, removeMe.y)
		}
	}
	return fromThese
}

func findAsteroid(asteroid *Asteroid, asteroids []*Asteroid) int {
	for i, current := range asteroids {
		if asteroid == current {
			return i
		}
	}
	return -1
}

func removeAsteroidAt(s int, slice []*Asteroid) []*Asteroid {
	result := make([]*Asteroid, len(slice)-1)
	result = append(slice[:s], slice[s+1:]...)
	return result
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
	// We calculate atan2(x,y) instead of (y,x), and we subtract that number from 180 to place
	// 0 degrees on top (12 o'clock)
	angle := 180 - (RADIAN_TO_DEGREE * math.Atan2(float64(other.x-this.x), float64(other.y-this.y)))
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

func (this *Asteroid) numVisibleAsteroids() int {
	if this.angles == nil {
		return 0
	}
	return len(this.angles)
}

func (this *Asteroid) manhattanDistance(other *Asteroid) int {
	return int(math.Abs(float64(other.x-this.x)) + math.Abs(float64(other.y-this.y)))
}

func (this *Asteroid) clearNearestVisible() {
	for k := range this.angles {
		delete(this.angles, k)
	}
}

func testAngles() {
	o := &Asteroid{0, 0, make(map[float64]*Asteroid)}
	a := &Asteroid{0, -1, nil}
	fmt.Printf("ABOVE angle to A %d,%d => %f == 0?\n", a.x, a.y, o.angleTo(a))

	a.x = 1
	a.y = -1
	fmt.Printf("UPPER RIGHT angle to A %d,%d => %f == 45?\n", a.x, a.y, o.angleTo(a))

	a.x = 1
	a.y = 0
	fmt.Printf("RIGHT angle to A %d,%d => %f == 90\n", a.x, a.y, o.angleTo(a))

	a.x = 1
	a.y = 1
	fmt.Printf("LOWER RIGHT angle to A %d,%d => %f == 135?\n", a.x, a.y, o.angleTo(a))

	a.x = 0
	a.y = 1
	fmt.Printf("BELOW angle to A %d,%d => %f == 180?\n", a.x, a.y, o.angleTo(a))

	a.x = -1
	a.y = 1
	fmt.Printf("LOWER LEFT angle to A %d,%d => %f == 225\n", a.x, a.y, o.angleTo(a))

	a.x = -1
	a.y = 0
	fmt.Printf("LEFT angle to A %d,%d => %f == 270\n", a.x, a.y, o.angleTo(a))

	a.x = -1
	a.y = -1
	fmt.Printf("angle to A %d,%d => %f == 315\n", a.x, a.y, o.angleTo(a))
}
