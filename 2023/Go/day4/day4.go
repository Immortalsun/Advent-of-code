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

//alias the numset type to a map of int keys to empty structs
type numSet map[int]struct{}

func (n numSet) add(setItem int) {
	n[setItem] = struct{}{}
}

//unused removal function but leaving it for
//documentation completeness
// func (n numSet) remove(setItem int) {
// 	delete(n, setItem)
// }

func (n numSet) contains(setItem int) bool {
	_,has := n[setItem]
	return has
}

type card struct {
	winningNumbers numSet
	score int
	id int
	winCount int
	instanceCount int
}

func (c *card) increaseScore() {
	if c.score == 0 {
		c.score = 1
	} else {
		c.score = c.score * 2
	}
	c.winCount++
}

func newCard(id int) *card {
	nCard := card{ id: id, winningNumbers: make(map[int]struct{}), score: 0, instanceCount: 1, winCount: 0 }
	return &nCard
}

func parseNumCollection(numLine string) []int {
	//split numbers on spaces
	numStrings := strings.Split(numLine, " ")
	//since we can have multiple spaces between numbers
	//we just buld and empty slice and apend, since the counts
	//will be different
	nums := make([]int, 0)
	//apply parsed numbers to slice
	for  _,numString := range numStrings {
		if numString == "" {
			continue
		}
		//clean spaces since they are not always single spaced
		numString = strings.TrimLeft(numString, " ")
		numString = strings.TrimRight(numString, " ")
		parsedNum, err := strconv.Atoi(numString)
		if err != nil {
			panic("Unable to parse number")
		}
		nums = append(nums, parsedNum)
	}

	return nums
}

func parseCard(cardLine string) *card {
	//remove unecessary 'game' prefix
	prefixCleaned := strings.TrimPrefix(cardLine, "Card ")

	//split the cleaned line on ':'
	//the resulting slice will have the first element as the id
	//and the second element will be all the numbers
	cardData := strings.Split(prefixCleaned, ": ")

	if len(cardData) != 2 {
		log.Fatal("Bad data line, no : found")
	}

	//convert the cardId
	cardId, err := strconv.Atoi(strings.TrimLeft(cardData[0]," "))

	if err != nil {
		panic("Failure to parse game id")
	}

	//we split the numbers colledtion on "|" to
	//divide into winning numbers (index O) and held
	//numbers (index 1)
	numbersCollections := strings.Split(cardData[1], " | ")

	winningNumbers := parseNumCollection(numbersCollections[0])
	heldNumbers := parseNumCollection(numbersCollections[1])

	//build card
	newCard := newCard(cardId)

	//add card winning numbers
	for _,winningNum := range winningNumbers {
		newCard.winningNumbers.add(winningNum)
	}

	//calculate card score
	for _,heldNum := range heldNumbers {
		if newCard.winningNumbers.contains(heldNum) {
			newCard.increaseScore()
		}
	}

	return newCard
}

func main() {
		//get path to input data file
		inputPath, err := filepath.Abs("../../../Data/day4Data.txt")
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

		cardCollection := make(map[int]card, 198)

		for scanner.Scan() {
			newCard := *parseCard(scanner.Text())
			cardCollection[newCard.id] = newCard
		}

		//part 1
		cardAccumulator := 0
		maxId := 198
		for _,card := range cardCollection {
			cardAccumulator += card.score
		}

		fmt.Printf("Total score of cards is: %d\n", cardAccumulator)

		//part 2
		for id:=1; id<199; id++ {
			//update instance counts of following
			//cards based on win count
			card, exists := cardCollection[id]
			if !exists {
				fmt.Printf("Card with id %d does not exist", id)
				panic("Bad card id")
			}

			if card.winCount < 1 {
				continue
			}

			nextId := card.id+1
			if nextId > maxId {
				break
			}

			for i:= 0; i<card.winCount; i++ {
				if nextId > maxId {
					break
				}
				//get card with the given id
				nextCard, hasCard := cardCollection[nextId]
				//if we find the card
				if hasCard {
					//update that cards instance count with
					//however many occurences in the current card
					nextCard.instanceCount += card.instanceCount
					//make sure to update the map with the updated card
					cardCollection[nextId] = nextCard
				}
				nextId++
			}
		}

		instanceAccumulator := 0
		for _,card := range cardCollection {
			instanceAccumulator += card.instanceCount
		}

		fmt.Printf("Total number of cards is: %d\n", instanceAccumulator)

}