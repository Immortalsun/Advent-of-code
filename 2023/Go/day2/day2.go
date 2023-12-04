package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"unicode/utf8"
)

type round struct {
	redCubeCount   int
	blueCubeCount  int
	greenCubeCount int
}

type game struct {
	id     int
	rounds []round
}

func (g *game) addRound(redCount int, greenCount int, blueCount int, index int) {
	g.rounds[index] = *newRound(redCount, greenCount, blueCount)
}

func (g *game) passesMaximumTest(maxRed int, maxGreen int, maxBlue int) bool {
	for _, round := range g.rounds {
		if round.redCubeCount > maxRed || round.blueCubeCount > maxBlue || round.greenCubeCount > maxGreen {
			return false
		}
	}
	return true
}

func (g *game) findMinimumRequiredSet() (minReqRed int, minReqGreen int, minReqBlue int) {
	mR, mG, mB := 0, 0, 0
	for _, round := range g.rounds {
		mR = max(round.redCubeCount, mR)
		mG = max(round.greenCubeCount, mG)
		mB = max(round.blueCubeCount, mB)
	}
	return mR, mG, mB
}

func (g *game) findPower() int {
	minReqRed, minReqGreen, minReqBlue := g.findMinimumRequiredSet()

	return minReqRed * minReqGreen * minReqBlue
}

func newGame(gameId int, roundCount int) *game {
	newGme := game{
		id:     gameId,
		rounds: make([]round, roundCount),
	}
	return &newGme
}

func newRound(redCount int, greenCount int, blueCount int) *round {
	newRnd := round{
		redCubeCount:   redCount,
		greenCubeCount: greenCount,
		blueCubeCount:  blueCount,
	}
	return &newRnd
}

func getCount(roundItem string) (int, rune) {
	//a round item is of the form num color.
	//so we can simply split on " "
	roundValues := strings.Split(roundItem, " ")
	firstRune, _ := utf8.DecodeRuneInString(roundValues[1])
	count, _ := strconv.Atoi(roundValues[0])
	return count, firstRune
}

func parseRound(roundLine string) (r int, g int, b int) {
	//a round line is delimited by commas, but its possible only one set was pulled
	//so first check if we have a comma, if not we can just grab the one value
	redCount, greenCount, blueCount := 0, 0, 0

	roundItems := strings.Split(roundLine, ", ")
	for _, roundStr := range roundItems {
		count, color := getCount(roundStr)

		switch color {
		case 'r':
			redCount = count
		case 'g':
			greenCount = count
		case 'b':
			blueCount = count
		}
	}
	return redCount, greenCount, blueCount
}

func parseGame(gameLine string) *game {
	//remove unecessary 'game' prefix
	prefixCleaned := strings.TrimPrefix(gameLine, "Game ")

	//split the cleaned line on ':'
	//the resulting slice will have the first element as the id
	//and the second element will be all the rounds
	gameData := strings.Split(prefixCleaned, ": ")

	if len(gameData) != 2 {
		log.Fatal("Bad data line, no : found")
	}

	//convert the gameId
	gameId, err := strconv.Atoi(gameData[0])

	if err != nil {
		panic("Failure to parse game id")
	}
	//get rounds by splitting the second item of the game data slice
	//further on semicolon and space, as that delimites items
	roundsCollection := strings.Split(gameData[1], "; ")

	if len(roundsCollection) < 1 {
		panic("Failure to parse rounds")
	}

	game := newGame(gameId, len(roundsCollection))

	//loop through the collection of rounds
	for i, round := range roundsCollection {
		r, g, b := parseRound(round)
		game.addRound(r, g, b, i)
	}

	fmt.Printf("Id: %d Rounds: %v\n", game.id, game.rounds)
	return game
}

func main() {

	//get path to input data file
	inputPath, err := filepath.Abs("../../../Data/day2Data.txt")
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
	scanner := bufio.NewScanner(file)

	//create games slice
	gamesCollection := make([]game, 100)

	gameIndex := 0
	//loop to scan the file
	for scanner.Scan() {
		//add each game into the collection
		gamesCollection[gameIndex] = *parseGame(scanner.Text())
		gameIndex = gameIndex + 1
	}

	//if scanner has an error
	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	//if we have no elements, something went wrong
	if len(gamesCollection) == 0 {
		log.Fatal("Parsing faled to produce games")
	}

	//part 1
	//declare max values for red, green, and blue cubes
	maxRed, maxGreen, maxBlue := 12, 13, 14

	//in part 1 the only way for a game to be possible is if
	//all reds across all rounds are less than or equal to max, etc
	sumPassingGameIds := 0
	for _, game := range gamesCollection {
		if game.passesMaximumTest(maxRed, maxGreen, maxBlue) {
			//add the game ids to the accumulator
			sumPassingGameIds = sumPassingGameIds + game.id
		} else {
			fmt.Printf("Game %d failed\n", game.id)
		}
	}

	fmt.Printf("Part 1 Solution: %d\n\n", sumPassingGameIds)
	//part 2
	//in part 2, we want to find the largest number of cubes for each color
	//so for each game, we need to go across the rounds and return a maxRed, maxBlue, and maxGreen
	//or, the minumum required set of cubes for the game to function
	//then the power of a the minimum set of cubes is equal to maxRed*maxBlue*maxGreen
	//then we simply accumulate all those power values together
	sumOfPowers := 0
	for _, game := range gamesCollection {

		power := game.findPower()
		fmt.Printf("Game %d cube power: %d\n", game.id, power)
		sumOfPowers = sumOfPowers + power
	}

	fmt.Printf("Part 2 Solution: %d", sumOfPowers)

}
