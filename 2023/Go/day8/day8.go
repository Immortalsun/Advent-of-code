package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"path/filepath"
	"strings"
	"unicode/utf8"
)

const goalString = "ZZZ"
const startString = "AAA"

type node struct {
	name string
	left string
	right string
	isGoal bool
	isStart bool
}

func newNode(newName string, newLeft string, newRight string) *node {
	nNode := node{ name: newName, left: newLeft, right: newRight}
	nNode.isGoal = newName == goalString
	nNode.isStart = newName == startString

	return &nNode
}

func parseNode(inputString string) *node {
	//split data on = to get name and direction pairs
	nodeData := strings.Split(inputString, " = ")
	name:= nodeData[0]
	directionalData := strings.TrimLeft(nodeData[1], "(")
	directionalData = strings.TrimRight(directionalData, ")")
	directions := strings.Split(directionalData, ", ")
	left := directions[0]
	right := directions[1]

	node:= newNode(name, left, right)

	return node
}

func greatestCommonDivisor(a int64, b int64) int64 {

	if b == 0 {
		return a
	}

	return greatestCommonDivisor(b, a % b)
}

func leastCommonMultiple(a int64, b int64) int64 {
	return (a * b) / greatestCommonDivisor(a, b)
}

func countStepsToEnd(startNode *node, directionLine string, nodeMap *map[string]node) int64 {
	steps := 0
	currNode := startNode
	for i := 0; i<len(directionLine); i++ {
		dir,_ := utf8.DecodeRuneInString(directionLine[i:])
		next := ""
		switch dir {
		case 'L':
			//get name of node on left
			next = currNode.left
		case 'R':
			//get name of node on right
			next = currNode.right
		}
		//move to the next node
		nextNode := (*nodeMap)[next]
		steps++
		//if we find the goal, done
		if nextNode.isGoal {
			break
		} else {
			//otherwise if we are at the end of
			//our list of directions, reset to the beginning
			if i+1 >= len(directionLine) {
				//iterator will add 1 at the end of the loop
				//so we set to -1 to get back to 0
				i = -1
			}
		}
		currNode = &nextNode
	}

	return int64(steps)
}

func main() {
	//get path to input data file
	inputPath, err := filepath.Abs("../../../Data/day8Data.txt")
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

	directionLine := ""
	nodeMap := make(map[string]node)
	readState := 0
	scanner := bufio.NewScanner(file)
	startNode := newNode("","","")

	for scanner.Scan() {
		if scanner.Text() == "" {
			readState++
			continue
		}
		if readState == 0 {
			directionLine = scanner.Text()
			continue
		}
		if readState == 1 {
			node := parseNode(scanner.Text())
			nodeMap[node.name] = *node

			if node.isStart {
				startNode = node
			}
		}
	}

	//part 1
	steps := countStepsToEnd(startNode, directionLine, &nodeMap)

	fmt.Printf("Total steps required to reach ZZZ: %d\n",steps)

	//part 2
	nodePaths := make([]node, 0)

	//find all startNodes and endNodes
	for k,v := range nodeMap {
		if strings.HasSuffix(k,"A") {
			v.isStart = true
			nodePaths = append(nodePaths, v)
		} else {
			if strings.HasSuffix(k,"Z") {
				v.isGoal = true
				nodeMap[k] = v
			}
		}
	}

	//run each start node to find its steps
	nodeSteps:= make([]int64, len(nodePaths))

	for i:=0; i<len(nodePaths); i++ {
		nodeSteps[i] = countStepsToEnd(&nodePaths[i], directionLine, &nodeMap)
	}

	//one we have steps we can find the least common multiple of all the numbers
	var lcm int64
	lcm = 0
	for i:=0; i<len(nodeSteps); i++ {
		if lcm == 0 {
			lcm = leastCommonMultiple(nodeSteps[i], nodeSteps[i+1])
			i = 1
			continue
		} else {
			lcm = leastCommonMultiple(lcm, nodeSteps[i])
		}
	}

	fmt.Printf("Total steps required to reach all %d end nodes: %d", len(nodePaths), lcm)

}