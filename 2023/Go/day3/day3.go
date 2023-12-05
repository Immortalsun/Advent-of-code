package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"unicode"
)

type coordinate struct {
	rowIdx int
	colIdx int
}

type gear struct {
	adjacentNums []int
	adjacentCount int
}

func newCoordinate(rowIndex int, colIndex int) *coordinate {
	coord := coordinate{ rowIdx: rowIndex, colIdx: colIndex }
	return &coord
}

func newGear(firstVal int) *gear {
	g := gear{
		adjacentCount: 1,
		adjacentNums: []int{ firstVal },
	}

	return &g;
}

func (g *gear) addAdjacentNum(num int) {
	g.adjacentNums = append(g.adjacentNums, num)
	g.adjacentCount++
}

func (g *gear) isValidGear() bool {
	return g.adjacentCount == 2
}

func (g *gear) getRatio() int {
	return g.adjacentNums[0] * g.adjacentNums[1]
}

func (c *coordinate) top() *coordinate {
	topCoord := newCoordinate(c.rowIdx-1, c.colIdx)
	return topCoord
}

func (c *coordinate) bottom() *coordinate {
	bottomCoord := newCoordinate(c.rowIdx+1, c.colIdx)
	return bottomCoord
}

func (c *coordinate) left() *coordinate {
	leftCoord := newCoordinate(c.rowIdx, c.colIdx-1)
	return leftCoord
}

func (c *coordinate) right() *coordinate {
	rightCoord := newCoordinate(c.rowIdx, c.colIdx+1)
	return rightCoord
}

func (c *coordinate) diagTopLeft() *coordinate {
	diagLeftTopCoord := newCoordinate(c.rowIdx-1, c.colIdx-1)
	return diagLeftTopCoord
}

func (c *coordinate) diagBottomleft() *coordinate {
	diagBottomLeftCoord := newCoordinate(c.rowIdx+1, c.colIdx-1)
	return diagBottomLeftCoord
}

func (c *coordinate) diagBottomRight() *coordinate {
	diagBottomRightCoord := newCoordinate(c.rowIdx+1, c.colIdx+1)
	return diagBottomRightCoord
}

func (c *coordinate) diagTopRight() *coordinate {
	diagTopRightCoord := newCoordinate(c.rowIdx-1, c.colIdx+1)
	return diagTopRightCoord
}

func (c *coordinate) inBounds(maxRowIndex int, maxColIndex int) bool {
	if c.colIdx < 0 || c.colIdx > maxColIndex {
		return false
	}

	if c.rowIdx < 0 || c.rowIdx > maxRowIndex {
		return false
	}

	return true
}

func runeIsDigit(r rune) bool {
	return unicode.IsDigit(r)
}

func runeIsSymbol(r rune) bool {
	return !runeIsDigit(r) && !runeIsPeriod(r)
}

func runeIsGearSymbol(r rune) bool {
	return r == '*'
}

func runeIsPeriod(r rune) bool {
	return r == '.'
}

func hasAdjacentSymbol(data *[][]rune, targetRow int, targetCol int) (bool, rune, coordinate) {

	maxRowIndex := len(*data) - 1
	if targetRow > maxRowIndex {
		fmt.Printf("Target row %d is out of bounds\n", targetRow)
		log.Fatal("Row out of bownds error")
	}

	dataRow := (*data)[targetRow]
	maxColIndex := len(dataRow) - 1

	if  targetCol > maxColIndex {
		fmt.Printf("Target column %d is out of bounds\n", targetCol)
		log.Fatal("Column out of bownds error")
	}

	//build coordinate to represent target
	targetCoord := newCoordinate(targetRow, targetCol)

	//build coordinates to represent targets
	top, bottom := targetCoord.top(), targetCoord.bottom()
	left, right := targetCoord.left(), targetCoord.right()
	diagTopLeft, diagTopRight := targetCoord.diagTopLeft(), targetCoord.diagTopRight()
	diagBottomLeft, diagBottomRight := targetCoord.diagBottomleft(), targetCoord.diagBottomRight()

	//to check a coordinate, we just need to make sure its in bounds
	//and then check the rune value there
	//we can get our cardinal direction bounds, if any are out we dont need to check
	//their corresponding diagonals
	topInBounds := top.inBounds(maxRowIndex, maxColIndex)
	bottomInBounds := bottom.inBounds(maxRowIndex, maxColIndex)
	leftInBounds := left.inBounds(maxRowIndex, maxColIndex)
	rightInBounds := right.inBounds(maxRowIndex, maxColIndex)
	checkTopDiag, checkBottomDiag := true, true

	if topInBounds {
		if tRuneVal := (*data)[top.rowIdx][top.colIdx]; runeIsSymbol(tRuneVal) {
			return true, tRuneVal, *top
		}
	} else {
		checkTopDiag = false
	}

	if bottomInBounds {
		if bRuneVal := (*data)[bottom.rowIdx][bottom.colIdx]; runeIsSymbol(bRuneVal) {
			return true, bRuneVal, *bottom
		}
	} else {
		checkBottomDiag = false
	}

	if leftInBounds {
		if lRuneVal := (*data)[left.rowIdx][left.colIdx]; runeIsSymbol(lRuneVal) {
			return true, lRuneVal, *left
		}
	}

	if rightInBounds {
		if rRuneVal := (*data)[right.rowIdx][right.colIdx]; runeIsSymbol(rRuneVal) {
			return true, rRuneVal, *right
		}
	}

	if checkTopDiag {

		if leftInBounds{
			if tlRuneVal := (*data)[diagTopLeft.rowIdx][diagTopLeft.colIdx]; runeIsSymbol(tlRuneVal) {
				return true, tlRuneVal, *diagTopLeft
			}
		}

		if rightInBounds{
			if tRRuneVal := (*data)[diagTopRight.rowIdx][diagTopRight.colIdx]; runeIsSymbol(tRRuneVal) {
				return true, tRRuneVal, *diagTopRight
			}
		}
	}

	if checkBottomDiag {

		if leftInBounds{
			if blRuneVal := (*data)[diagBottomLeft.rowIdx][diagBottomLeft.colIdx]; runeIsSymbol(blRuneVal) {
				return true, blRuneVal, *diagBottomLeft
			}
		}

		if rightInBounds{
			if bRRuneVal := (*data)[diagBottomRight.rowIdx][diagBottomRight.colIdx]; runeIsSymbol(bRRuneVal) {
				return true, bRRuneVal, *diagBottomRight
			}
		}
	}


	return false, ' ', *newCoordinate(-1,-1)
}

func tryAccumulatePartNumber(positions *[]int, builder *strings.Builder, data *[][]rune, accumulator *int, row int) {
	for i := 0; i<len(*positions);  i++ {
		//if we have a position to check
		if pos := (*positions)[i]; pos >= 0 {
			//check adjacency at that position
			if hasAdjacency,_,_ := hasAdjacentSymbol(data, row, (*positions)[i]); hasAdjacency {
				//if we find adjacency in any of those positions,
				//we can accumulate our value and leave the loop
				partNum,_ := strconv.Atoi(builder.String())
				*accumulator += partNum
				break
			}
		}
	}

	(*positions)[0] = -1
	(*positions)[1] = -1
	(*positions)[2] = -1
	builder.Reset()
}

func tryPartNumberAndGearInfo(positions *[]int, builder *strings.Builder, data *[][]rune, row int) (partnum int, gCoord coordinate) {
	partNumber := -1
	var symbolCoordinate coordinate

	for i := 0; i<len(*positions);  i++ {
		//if we have a position to check
		if pos := (*positions)[i]; pos >= 0 {
			//check adjacency at that position
			if hasAdjacency, adjacentSymbol, symbolCoord := hasAdjacentSymbol(data, row, (*positions)[i]); hasAdjacency {
				//if we find specifically a gear adjacency in any position
				if runeIsGearSymbol(adjacentSymbol) {
					partNum,_ := strconv.Atoi(builder.String())
					partNumber = partNum
					symbolCoordinate = symbolCoord
					break;
				}
			}
		}
	}

	(*positions)[0] = -1
	(*positions)[1] = -1
	(*positions)[2] = -1
	builder.Reset()

	return partNumber, symbolCoordinate
}

func updateGearCollection(gearColl *map[coordinate]gear, newCoord *coordinate, partNum int) {
	//don't do anything if we have an invalid part number
	if partNum < 0 {
		return;
	}
	//if we have a gear with that coordinate
	gear,present := (*gearColl)[*newCoord]
	if present {
		gear.addAdjacentNum(partNum)
		(*gearColl)[*newCoord] = gear
	} else {
		(*gearColl)[*newCoord] = *newGear(partNum)
	}
}


func main(){
	//get path to input data file
	inputPath, err := filepath.Abs("../../../Data/day3Data.txt")
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

	schematicData := make([][]rune, 140)
	//create a scanner object to read the file line by line
	scanner := bufio.NewScanner(file)

	rowIdx, colIdx := 0,0
	for scanner.Scan() {
		//add each line of runes to the collection
		schematicLine := scanner.Text()
		schematicData[rowIdx] = make([]rune, 140)

		for _,runeVal := range schematicLine {
			schematicData[rowIdx][colIdx] = runeVal
			colIdx += 1
		}
		rowIdx += 1
		colIdx = 0
	}

	//part 1
	//numbers are either 1, 2, or 3 runes long, and if a
	//symbol is anywhere within 1 index of any part of the number
	//the entire number is valid
	partAccumulator := 0
	for row := 0; row < len(schematicData); row++ {
		schematicRow := schematicData[row]

		//init digit state counters
		var builder strings.Builder
		foundDigits := false
		//we have at most 3 indices in a given set that hold digits
		digitPositions := make([]int, 3)
		digitIdx := 0
		digitPositions[0] = -1
		digitPositions[1] = -1
		digitPositions[2] = -1

		for col := 0; col < len(schematicRow); col++ {

			//if we are reading anything other than a digit
			if !runeIsDigit(schematicRow[col])  {
				//if we have read digits and have reached something
				//that is not a digit, we need to check the digit positions
				//for adjacency
				if foundDigits {
					tryAccumulatePartNumber(&digitPositions, &builder, &schematicData, &partAccumulator, row)
					digitIdx = 0
					foundDigits = false
				}
			} else {
				//add our digit to the builder
				builder.WriteRune(schematicRow[col])
				digitPositions[digitIdx] = col
				foundDigits = true
				digitIdx++

				//handle case where digit is at the end of the line
				if col + 1 >= len(schematicRow) {
					tryAccumulatePartNumber(&digitPositions, &builder, &schematicData, &partAccumulator, row)
					digitIdx = 0
					foundDigits = false
				}
			}
		}
	}

	fmt.Printf("Accumulated part numbers: %d\n", partAccumulator)

	//part 2
	//gear markers * have to be within adjacency of exactly 2 numbers to be valid
	//we can use the code from part 1 which retrieves numbers, but add the ability
	//to detect if the adjacent symbol is a gear symbol, and record the position of
	//that symbol. as we scan the next line, we can perform the same function, and if
	//the adjacency is the same symbol, add that number to the gears collection
	//we can then find the gears who have exactly 2 adjacent numbers, and accumulate
	//the ratios (two gear numbers multiplied)
	gearCollection := make(map[coordinate]gear)

	for row := 0; row < len(schematicData); row++ {
		schematicRow := schematicData[row]

		//init digit state counters
		var builder strings.Builder
		foundDigits := false
		//we have at most 3 indices in a given set that hold digits
		digitPositions := make([]int, 3)
		digitIdx := 0
		digitPositions[0] = -1
		digitPositions[1] = -1
		digitPositions[2] = -1

		for col := 0; col < len(schematicRow); col++ {

			//if we are reading anything other than a digit
			if !runeIsDigit(schematicRow[col])  {
				//if we have read digits and have reached something
				//that is not a digit, we need to check the digit positions
				//for adjacency
				if foundDigits {
					partNum, gearCoord := tryPartNumberAndGearInfo(&digitPositions, &builder, &schematicData, row)
					updateGearCollection(&gearCollection, &gearCoord, partNum)
					digitIdx = 0
					foundDigits = false
				}
			} else {
				//add our digit to the builder
				builder.WriteRune(schematicRow[col])
				digitPositions[digitIdx] = col
				foundDigits = true
				digitIdx++

				//handle case where digit is at the end of the line
				if col + 1 >= len(schematicRow) {
					partNum, gearCoord := tryPartNumberAndGearInfo(&digitPositions, &builder, &schematicData, row)
					updateGearCollection(&gearCollection, &gearCoord, partNum)
				}
			}
		}
	}

	gearAccumulator := 0
	for _,gear := range gearCollection {
		if gear.isValidGear() {
			gearAccumulator += gear.getRatio()
		}
	}

	fmt.Printf("Accumulated gear ratios: %d", gearAccumulator)

}