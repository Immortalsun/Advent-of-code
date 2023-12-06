package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"path/filepath"
	"strconv"
	"strings"
)

type race struct {
	raceTime int64
	recordDist int64
	waysToWin int64
}

func newRace(time int64, record int64) *race {
	nRace := race{ raceTime: time, recordDist: record, waysToWin: 0 }
	return &nRace
}

func (r *race) calculateWaysToWin() {
	var heldTime int64
	for heldTime = 1; heldTime < r.raceTime; heldTime++ {
		travelTime := r.raceTime - heldTime
		if travelTime * heldTime > r.recordDist {
			r.waysToWin++
		}
	}
}

func parseRaces(timeLine string, distLine string) *[]race {
	timeParts := strings.Split(timeLine, ": ")
	distParts := strings.Split(distLine, ": ")
	timesString := timeParts[1]
	distsString := distParts[1]

	timesCollection := strings.Split(timesString, " ")
	distsCollection := strings.Split(distsString, " ")

	raceCollection := make([]race, 0)
	for i := 0; i<len(timesCollection); i++ {
		time,_ := strconv.ParseInt(timesCollection[i], 10, 64)
		distance,_ := strconv.ParseInt(distsCollection[i], 10, 64)
		race := newRace(time, distance)
		race.calculateWaysToWin()
		raceCollection = append(raceCollection, *race)
	}

	return &raceCollection
}

func parseBigRace(timeLine string, distLine string) *race {
	timeParts := strings.Split(timeLine, ": ")
	distParts := strings.Split(distLine, ": ")
	timesString := timeParts[1]
	distsString := distParts[1]

	timesCollection := strings.Split(timesString, " ")
	distsCollection := strings.Split(distsString, " ")

	bigTime := strings.Join(timesCollection, "")
	bigDist := strings.Join(distsCollection, "")

	time,_ := strconv.ParseInt(bigTime, 10, 64)
	distance,_ := strconv.ParseInt(bigDist, 10, 64)
	race := newRace(time, distance)
	race.calculateWaysToWin()

	return race
}

func main() {
	//get path to input data file
	inputPath, err := filepath.Abs("../../../Data/day6Data.txt")
	if err != nil {
		fmt.Println("Relative path retrieval failed")
	} else {
		fmt.Println(inputPath)
	}

	file, err := os.Open(inputPath)
	if err != nil {
		log.Fatal(err)
	}

	//defer closing the file until program exit
	defer file.Close()

	//create a scanner object to read the file line by line
	timeLine := ""
	distLine := ""
	readIndex := 0
	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		if readIndex == 0 {
			timeLine = scanner.Text()
			readIndex++
		} else {
			distLine = scanner.Text()
		}
	}

	//part 1
	races := parseRaces(timeLine, distLine)

	var winAccumulator int64
	winAccumulator = 1
	for _,race := range *races {
		winAccumulator = winAccumulator * race.waysToWin
	}

	fmt.Printf("Accumulated ways to win: %d\n", winAccumulator)

	//part 2
	bigRace := parseBigRace(timeLine, distLine)
	fmt.Printf("Ways to win big race: %d\n", bigRace.waysToWin)

}