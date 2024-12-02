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

type reading struct {
	history []int64
	predictedVal int64
}

func newReading(readingCount int) *reading {
	reading := reading{ history: make([]int64, readingCount), predictedVal: 0}
	return &reading
}

func (r *reading) calculatePredictedValue(){
	r.predictedVal = getNextSequenceValue(r.history)
}

func getNextSequenceValue(seq []int64) int64 {
	diffValues := make([]int64, len(seq)-1)

	allZeroDiffs := true
	for i:=0; i<len(diffValues); i++ {
		diffVal := seq[i+1] - seq[i]
		diffValues[i] = diffVal
		allZeroDiffs = diffVal == 0
	}

	if allZeroDiffs {
		return  seq[len(seq)-1]
	}

	return seq[len(seq)-1]+getNextSequenceValue(diffValues)
}

func parseReading(intputLine string) *reading {
	numCollection := strings.Split(intputLine, " ")
	reading:= newReading(len(numCollection))
	for i,numStri := range numCollection {
		val,_ := strconv.ParseInt(numStri, 10, 64)
		reading.history[i] = val
	}

	reading.calculatePredictedValue()
	return reading
}

func main() {
	inputPath, err := filepath.Abs("../../../Data/day9Data.txt")
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
	readings := make([]reading, 0)
	for scanner.Scan() {
		readings = append(readings, *parseReading(scanner.Text()))
	}

	var accumulator int64
	accumulator = 0
	for _,read := range readings {
		accumulator += read.predictedVal
	}

	fmt.Printf("Predicted value total: %d", accumulator)

}