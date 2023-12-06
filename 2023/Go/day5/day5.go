package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"time"
)

type rangeMap struct {
	sourceStart int64
	destinationStart int64
	rangeLength int64
	destinationEnd int64
	sourceEnd int64
}

type seedRange struct {
	rangeStart int64
	rangeEnd int64
	rangeLength int64
}

func newRangeMap(sStart int64, dStart int64, length int64) *rangeMap {
	newRange := rangeMap {
		sourceStart: sStart,
		destinationStart: dStart,
		rangeLength: length,
		destinationEnd: (dStart+length)-1,
		sourceEnd: (sStart+length)-1,
	}
	return &newRange
}

func newSeedRange(start int64, length int64) *seedRange {
	newSeedRange := seedRange {
		rangeStart: start,
		rangeLength: length,
		rangeEnd: (start+length)-1,
	}
	return &newSeedRange
}

func (r *rangeMap) isInSourceRange(item int64) bool {
	return item >= r.sourceStart && item <= r.sourceEnd
}

func (r *rangeMap) getMappedDestination(itemInSourceRange int64) int64 {
	distFromStart := itemInSourceRange - r.sourceStart
	return r.destinationStart + distFromStart
}

func getSplitNumsCollection(numString string) *[]int64 {
	splitStrings := strings.Split(numString, " ")
	retSlice := make([]int64, 3)
	for i,splitNumStr := range splitStrings {
		num, err := strconv.ParseInt(splitNumStr, 10, 64)
		if err != nil {
			fmt.Printf("Range num %s is bigger than int64 max", splitNumStr)
			panic("int64 parse failure")
		}
		retSlice[i] = num
	}
	return &retSlice
}

func getRangeMapFromInt64Slice(intSlice *[]int64) *rangeMap {
	//index 0 is destination start
	//index 1 is source start
	//index 2 is length
	return newRangeMap((*intSlice)[1],(*intSlice)[0],(*intSlice)[2])
}

func tryFindMappedValueInRangeMaps(rangeMaps *[]rangeMap, input int64) int64 {
	retVal := input
	for i:=0; i<len(*rangeMaps); i++ {
		testMap := (*rangeMaps)[i]
		if testMap.isInSourceRange(input) {
			retVal = testMap.getMappedDestination(retVal)
			break
		}
	}
	return retVal
}

func main() {
	//get path to input data file
	inputPath, err := filepath.Abs("../../../Data/day5Data.txt")
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

	//readState int controls which colletion we fill
	// 0 seeds
	// 1 seedToSoil
	// 2 soilToFert
	// 3 fertToWater
	// 4 waterToLight
	// 5 lightToTemp
	// 6 tempToHumid
	// 7 humidToLoc
	readState := 0
	seeds := make([]int64, 20)
	seedToSoil := make([]rangeMap, 0)
	soilToFertilizer := make([]rangeMap, 0)
	fertilizerToWater := make([]rangeMap, 0)
	waterToLight := make([]rangeMap, 0)
	lightToTemp := make([]rangeMap, 0)
	tempToHumidity := make([]rangeMap, 0)
	humidityToLoc := make([]rangeMap, 0)


	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			readState++
			continue
		}
		if strings.Contains(line, "map") {
			continue
		}
		switch readState {
		case 0:
			//split on :
			seedsStrColl := strings.Split(line, ": ")
			if len(seedsStrColl) < 2 {
				panic("Error splitting seeds")
			}
			seedStrings := strings.Split(seedsStrColl[1], " ")
			for i,seedStr := range seedStrings {
				seed, err := strconv.ParseInt(seedStr, 10, 64)
				if err != nil {
					fmt.Printf("Seed %s is bigger than int64 max", seedStr)
					panic("int64 parse failure")
				}
				seeds[i] = seed
			}
		case 1:
			//all other lines are just numbers separated by spaces
			//so we can jus use our utility method
			seedToSoilNums := getSplitNumsCollection(line)
			seedToSoil = append(seedToSoil, *getRangeMapFromInt64Slice(seedToSoilNums))
		case 2:
			//all other lines are just numbers separated by spaces
			//so we can jus use our utility method
			soilToFertNums := getSplitNumsCollection(line)
			soilToFertilizer = append(soilToFertilizer, *getRangeMapFromInt64Slice(soilToFertNums))
		case 3:
			fertToWaterNums := getSplitNumsCollection(line)
			fertilizerToWater = append(fertilizerToWater, *getRangeMapFromInt64Slice(fertToWaterNums))
		case 4:
			waterToLightNums := getSplitNumsCollection(line)
			waterToLight = append(waterToLight, *getRangeMapFromInt64Slice(waterToLightNums))
		case 5:
			lightToTempNums := getSplitNumsCollection(line)
			lightToTemp = append(lightToTemp, *getRangeMapFromInt64Slice(lightToTempNums))
		case 6:
			tempToHumidityNums := getSplitNumsCollection(line)
			tempToHumidity = append(tempToHumidity, *getRangeMapFromInt64Slice(tempToHumidityNums))
		case 7:
			humidityToLocNums := getSplitNumsCollection(line)
			humidityToLoc = append(humidityToLoc, *getRangeMapFromInt64Slice(humidityToLocNums))
		}
	}

	//part 1
	//after we have populated our maps, we can go through our seeds to trace locations
	var lowestLocation int64
	//set lowest location to int64 max
	lowestLocation = 9223372036854775807
	for _,testSeed :=  range seeds {
		//initialize soil to seed value
		//as if we don't find a mapped soil,
		//the soil value is the same as input seed value
		soil := tryFindMappedValueInRangeMaps(&seedToSoil, testSeed)

		//use soil to find fertilizer
		fertilizer := tryFindMappedValueInRangeMaps(&soilToFertilizer, soil)

		//use fertilizer to find water
		water := tryFindMappedValueInRangeMaps(&fertilizerToWater, fertilizer)

		//use water to find light
		light := tryFindMappedValueInRangeMaps(&waterToLight, water)

		//use light to find temp
		temp := tryFindMappedValueInRangeMaps(&lightToTemp, light)

		//use temp to find humidity
		humidity := tryFindMappedValueInRangeMaps(&tempToHumidity, temp)

		//use humiditiy t find location
		location := tryFindMappedValueInRangeMaps(&humidityToLoc, humidity)

		//update lowest location
		lowestLocation = min(lowestLocation, location)
	}

	fmt.Printf("Lowest location number: %d\n",lowestLocation)

	//part 2
	//in the seeds collection, pairs represent ranges
	//the real implementation is binary search
	//but lets just check em boss
	startTime := time.Now()
	fmt.Printf("Part 2 started at %s\n", startTime)

	seedRanges := make([]seedRange, 0)
	for i := range seeds {
		if i < len(seeds) && i%2 == 0 {
			seedRanges = append(seedRanges, *newSeedRange(seeds[i], seeds[i+1]))
		}
	}

	var lowestRangeLocation int64
	lowestRangeLocation = 9223372036854775807
	//go through our ranges
	for _,seedRangeItem := range seedRanges {
		for seed := seedRangeItem.rangeStart; seed <= seedRangeItem.rangeEnd; seed++ {

			soil := tryFindMappedValueInRangeMaps(&seedToSoil, seed)

			//use soil to find fertilizer
			fertilizer := tryFindMappedValueInRangeMaps(&soilToFertilizer, soil)

			//use fertilizer to find water
			water := tryFindMappedValueInRangeMaps(&fertilizerToWater, fertilizer)

			//use water to find light
			light := tryFindMappedValueInRangeMaps(&waterToLight, water)

			//use light to find temp
			temp := tryFindMappedValueInRangeMaps(&lightToTemp, light)

			//use temp to find humidity
			humidity := tryFindMappedValueInRangeMaps(&tempToHumidity, temp)

			//use humiditiy t find location
			location := tryFindMappedValueInRangeMaps(&humidityToLoc, humidity)

			//update lowest location
			lowestRangeLocation = min(lowestRangeLocation, location)
		}
	}

	fmt.Printf("Lowest seed range location number: %d\n", lowestRangeLocation)
	fmt.Printf("Part 2 finished at %s\n", time.Since(startTime))
}