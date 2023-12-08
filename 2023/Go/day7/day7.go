package main

import (
	"bufio"
	"cmp"
	"fmt"
	"log"
	"os"
	"path/filepath"
	"regexp"
	"slices"
	"strconv"
	"strings"
)

type numSet map[int]struct{}

func (n numSet) add(setItem int) {
	n[setItem] = struct{}{}
}

func (n numSet) contains(setItem int) bool {
	_,has := n[setItem]
	return has
}

type card struct {
	label string
	strength int
}

type hand struct {
	cards []card
	category int
	bid int
	handLabel string
}

func newCard(inputLabel string, useJokers bool) *card {
	outCard := card{label: inputLabel}
	strVal, err := strconv.Atoi(inputLabel)
	if err == nil {
		outCard.strength = strVal
	} else {
		switch inputLabel {
		case "A":
			outCard.strength = 14
		case "K":
			outCard.strength = 13
		case "Q":
			outCard.strength = 12
		case "J":
			if useJokers {
				outCard.strength = 1
			} else {
				outCard.strength = 11
			}
		case "T":
			outCard.strength = 10
		}
	}
	return &outCard
}

func newHand(bid int, handLabel string) *hand {
	outHand := hand{ cards: make([]card, 5), bid: bid, handLabel: handLabel}
	return &outHand
}

func (h *hand) addCard(cardToAdd card, pos int){
	h.cards[pos] = cardToAdd
}

func (h *hand) categorizeHand(fullHandData string) {
	if len(h.cards) != 5 {
		panic("Number of cards in hand is not 5")
	}
	//categories range from 7 to 1, where 7 is five of a kind
	//and 1 is high-card (all unique)
	dupSet := make(numSet)
	for i,card := range h.cards {
		if dupSet.contains(i) {
			continue
		}
		//check for occurences of each card in the full string
		r,_ := regexp.Compile(card.label)
		val := r.FindAllStringIndex(fullHandData, -1)

		switch len(val) {
		case 5: //five of a kind, no other options
			h.category = fiveOfAKind
			return
		case 4: //four of a kind, no other options
			h.category = fourOfAKind
			return
		case 3: //we have three of one card, either a full house or three of a kind
			for j:= 0; j<len(val); j++ {
				dupSet.add(val[j][0])
			}
			for y:= 0; y<len(h.cards); y++ {
				if dupSet.contains(y) {
					continue
				}
				//find first non match index and
				//run regex on that character
				//if we have to occurences, we have a full house,
				//else three of a kind
				n,_ := regexp.Compile(h.cards[y].label)
				subMatch := n.FindAllStringIndex(fullHandData, -1)
				if len(subMatch) == 2 {
					h.category = fullHouse
					return
				} else {
					h.category = threeOfAKind
					return
				}
			}
			case 2: // we have a pair, we are either full house, two pair or one pair
				for j:= 0; j<len(val); j++ {
					dupSet.add(val[j][0])
				}

				for y:= 0; y<len(h.cards); y++ {
					if dupSet.contains(y) {
						continue
					}
					//find first non match index and
					//run regex on that character
					//if we have to occurences, we have two pair
					n,_ := regexp.Compile(h.cards[y].label)
					subMatch := n.FindAllStringIndex(fullHandData, -1)
					//if we found 3 of the same card, full house
					if len(subMatch) == 3 {
						h.category = fullHouse
						return
					}
					//if we found another pair, we can consider two pair
					if len(subMatch) == 2 {
						h.category = twoPair
						return
					}
				}
				//if we didnt find another pair then we have one pair
				h.category = onePair
				return
			case 1:
				//keep searching for pairs
		}
	}

	//if we never found pairs and short circuited early, we have all unique carcs
	h.category = highCard
}

/*hand categories
7 - five of a kind
6 - four of a kind
5 - full house
4 - three of a kind
3 - two pair
2 - one pair
1 - high card
*/
const fiveOfAKind = 7
const fourOfAKind = 6
const fullHouse = 5
const threeOfAKind = 4
const twoPair = 3
const onePair = 2
const highCard = 1

func (h *hand) categorizeHandWithJokers(fullHandData string) {
	if len(h.cards) != 5 {
		panic("Number of cards in hand is not 5")
	}

	jokerSet := make(numSet)
	jRegex,_ := regexp.Compile("J")
	jVal := jRegex.FindAllStringIndex(fullHandData, -1)
	jokerCount := len(jVal)
	hasJokers := jokerCount > 0

	//use normal categorization if we dont have jokers
	if !hasJokers {
		h.categorizeHand(fullHandData)
		return
	}

	for j:= 0; j<len(jVal); j++ {
		jokerSet.add(jVal[j][0])
	}

	pairSet := make(numSet)
	//if we have 5 or 4 jokers, five of a kind
	if jokerCount == 5 || jokerCount == 4 {
		h.category = fiveOfAKind
		return
	}

	//if we have 3 jokers
	if jokerCount == 3 {
		//if the non joker is a pair, five of kind
		//otherwise four of a kind
		for i,card := range h.cards {
			if jokerSet.contains(i) {
				continue
			}
			//check for occurences of each card in the full string
			r,_ := regexp.Compile(card.label)
			val := r.FindAllStringIndex(fullHandData, -1)

			if len(val) == 2 {
				h.category = fiveOfAKind
				return
			}

			h.category = fourOfAKind
			return
		}
	}

	//if we have two jokers
	if jokerCount == 2 {

		for i,card := range h.cards {
			if jokerSet.contains(i) {
				continue
			}
			//check for occurences of each card in the full string
			r,_ := regexp.Compile(card.label)
			val := r.FindAllStringIndex(fullHandData, -1)

			//if we find 3 ocurrences of another card, we have
			//five of a kind
			if len(val) == 3 {
				h.category = fiveOfAKind
				return
			}

			//if we find another pair, we have 4 of a kind
			if len(val) == 2 {
				h.category = fourOfAKind
				return
			}
		}
		//if we found no other pairs, we have 3 of a kind
		h.category = threeOfAKind
		return
	}

	//if we have 1 joker
	if jokerCount == 1 {

		for i,card := range h.cards {
			if jokerSet.contains(i) || pairSet.contains(i) {
				continue
			}
			//check for occurences of each card in the full string
			r,_ := regexp.Compile(card.label)
			val := r.FindAllStringIndex(fullHandData, -1)

			//if we find four of the same card, five of a kind
			if len(val) == 4 {
				h.category = fiveOfAKind
				return
			}

			//if we find three of the same card, four of a kind
			if len(val) == 3{
				h.category = fourOfAKind
				return
			}

			//if we find a pair
			if len(val) == 2 {
				//add it to our pair set
				for j:= 0; j<len(val); j++ {
					pairSet.add(val[j][0])
				}
			}
		}

		//if we have other pairs
		if len(pairSet) > 0 {
			//length by 2 to get pair cound
			pairCount := len(pairSet)/2
			//if we found one set of pairs, 3 of a kind
			if pairCount == 1 {
				h.category = threeOfAKind
				return
			}
			//if we found 2 sets of pairs
			//with a joker that means we have a full house
			if pairCount == 2 {
				h.category = fullHouse
				return
			}
		}
	}

	//if we never found pairs and short circuited early, we have all unique cards,
	//but since we have jokers, we can assume we have at least one pair
	h.category = onePair
}


func parseHand(inputHandStr string, useJokers bool) *hand {
	handData := strings.Split(inputHandStr, " ")
	bid, _ := strconv.Atoi(handData[1])
	hand := newHand(bid, handData[0])

	for i,r := range handData[0] {
		card := newCard(string(r), useJokers)
		hand.addCard(*card, i)
	}
	if useJokers {
		hand.categorizeHandWithJokers(handData[0])
	} else {
		hand.categorizeHand(handData[0])
	}

	return hand
}

func main(){
	//get path to input data file
	inputPath, err := filepath.Abs("../../../Data/day7Data.txt")
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

	scanner := bufio.NewScanner(file)

	hands := make([]hand, 0)

	for scanner.Scan() {
		hands = append(hands, *parseHand(scanner.Text(), true))
	}

	//sort hands based on rules
	slices.SortFunc(hands, func(a, b hand) int {
		if a.category == b.category {
			//if hands are in the same category
			//compare cards
			for i:=0; i<len(a.cards); i++ {
				if a.cards[i].strength == b.cards[i].strength {
					continue
				} else {
					aStrength, bStrength := a.cards[i].strength, b.cards[i].strength
					return cmp.Compare(aStrength, bStrength)
				}
			}
		}

		aCategory, bCategory := a.category, b.category
		return cmp.Compare(aCategory, bCategory)
	})

	//print sorted hands
	for _,hand := range hands {
		fmt.Println(strings.Join([]string{hand.handLabel,fmt.Sprint(hand.category)}," - "))
	}


	totalWinningsAccumulator := 0
	//a hands rank is just its index + 1 in the sorted list
	for i:=0; i<len(hands); i++ {
		rank := i+1
		totalWinningsAccumulator += rank * hands[i].bid
	}

	fmt.Printf("Total Winnings are: %d", totalWinningsAccumulator)
}