// ConsoleApplication1.cpp : This file contains the 'main' function. Program execution begins and ends there.
//

#include <iostream>
#include <stack>
#include <map>
#include <fstream>
#include <string>
#include "FileTree.h"
#include "IndexMap.h"
#include "MonkeyBusiness.h"
#include "RopeSolver.h"
#include "PathSolver.h"

using namespace std;

void Problem6(bool doPart1 = true) {
    ifstream inputStream(R"(Problem6Input.txt)");
    if(inputStream.is_open())
    {
        string inputLine;
        string fourCharMarker;
        string messageMarker;
        bool done = false;
        size_t markerPosition = 0;
        size_t messagePosition = 0;
	    while(inputStream && !done)
	    {
            std::getline(inputStream, inputLine);
            if(inputLine.empty())
            {
                throw;
            }

            if(doPart1)
            {
                for (size_t i = 0; i < inputLine.length(); i++)
                {
                    for (size_t j = i; j < i + 4; j++)
                    {
                        if (fourCharMarker.find(inputLine[j]) == string::npos)
                        {
                            fourCharMarker += inputLine[j];
                        }
                        else
                        {
                            i = j;
                            break;
                        }
                    }

                    if (fourCharMarker.length() == 4)
                    {
                        markerPosition = i + 4;//4 to get to end of marker
                        break;
                    }

                    fourCharMarker = "";
                }

                cout << "Marker position is " + to_string(markerPosition)+"\n";
                done = true;
            }
            else
            {
                for (size_t i = 0; i < inputLine.length(); i++)
                {
                    for (size_t j = i; j < i + 14 && j < inputLine.length(); j++)
                    {
                        if (messageMarker.find(inputLine[j]) == string::npos)
                        {
                            messageMarker += inputLine[j];
                        }
                        else
                        {
                            i = j;
                            break;
                        }
                    }

                    if (messageMarker.length() == 14)
                    {
                        messagePosition = i + 14;//4 to get to end of marker
                        break;
                    }

                    messageMarker = "";
                }

                cout << "Message position is " + to_string(messagePosition);
                done = true;
            }

	    }
        inputStream.close();
    }
}

void Problem7()
{
    ifstream inputStream(R"(Problem7Input.txt)");
    if (inputStream.is_open())
    {
        string inputLine;
        FileTree tree{};
        while (inputStream)
        {
            std::getline(inputStream, inputLine);

            if(inputLine.empty())
            {
                break;
            }

            //command declaration line
            if(inputLine[0] == '$')
            {
                string command = inputLine.substr(2, 2);
                if(command == "cd")
                {
                    string targetDirName = inputLine.substr(5);
                    if(targetDirName == "..")
                    {
                        tree.NavigateCurrentToParent();
                        cout << "Navigating up one level to " + tree.GetCurrentNodeName() +"\n";
                    }
                    else
                    {
                        tree.ChangeCurrentToNode(targetDirName);
                        cout << "Changing to directory "+targetDirName+"\n";
                    }
                }
                //if command is 'ls' we don't need to do anything because we are going to begin adding items at the next line
            }
            //directory listing
            else if (inputLine[0] == 'd')
            {
                string dirListingName = inputLine.substr(4);
                DirectoryNode dirNode(dirListingName);
                tree.AddDirectoryChildToCurrentNode(dirNode);
                cout << "Adding new directory " + dirListingName + " to " + tree.GetCurrentNodeName() + "\n";
            }
            //file listing
            else
            {
                size_t fileSpaceDividerPos = inputLine.find(' ');
                string fileSizeStr = inputLine.substr(0, fileSpaceDividerPos);
                string fileName = inputLine.substr(fileSpaceDividerPos + 1);
                size_t fileSizeInt = stoul(fileSizeStr);
                FileNode fileNode(fileName, fileSizeInt);
                tree.AddFileChildToCurrentNode(fileNode);
                cout << "Adding new file " + fileName + " to " + tree.GetCurrentNodeName() + "\n";
            }

            inputLine = "";
        }

        cout << "\n";
        cout << "\n";

        inputStream.close();
        TreeTraverser traverser{};
        traverser.GetTotalDirectorySizesUnderLimit(tree.GetRoot());
        size_t totalDirectorySize = traverser.totalDirectorySize;

        cout << "\n";
        cout << "\n";

        traverser.GetSmallestDirectorOverSizeLimit(tree.GetRoot());
        size_t minimumRemovalSize = traverser.smallestSizeOverLimit;

        cout << "Total size is " + to_string(totalDirectorySize) + "\n";
        cout << "Smallest size to remove for space is " + to_string(minimumRemovalSize) + "\n";

    }
}

void Problem8()
{
    ifstream inputStream(R"(Problem8Input.txt)");
    if (inputStream.is_open())
    {
        IndexMap map{};
        string inputLine;
        int rowIndex = 0;
        size_t visibleTreeCount = 0;
        size_t highestScenicScore = 0;
        while(inputStream)
        {
            std::getline(inputStream, inputLine);

            if (inputLine.empty())
            {
                break;
            }

            for(size_t col = 0; col<inputLine.length(); col++)
            {
                char& inputChar = inputLine[col];
                int inputInt = inputChar - '0'; //convert char to int representation
                map.AddIndexItem(rowIndex, col, inputInt);
            }

            inputLine = "";
            rowIndex++;
        }

        auto& treesCollection = map.GetIndexItems();
        const int maxRowLength = static_cast<int>(treesCollection.size());
        for (int row = 0; row < maxRowLength; row++)
        {
            auto& rowCollection = treesCollection[row];
            const int maxColLength = static_cast<int>(rowCollection.size());
            for (int col = 0; col < maxColLength; col++)
            {
                //items at row 0 are always visible
                //items in last row are also always visible
                if(row == 0 || row == maxRowLength-1)
                {
                    visibleTreeCount++;
                    continue;
                }

                //items in col 0 are always visible
                //items in the last column are always visible
            	if(col == 0 || col == maxColLength-1)
                {
                    visibleTreeCount++;
                    continue;
                }

                //an item is visible if any ONE of the following conditions are true
					//All items in previous columns are smaller (row,col->0)
					//All the items in the next columns are smaller (row,col->maxColLength-1)
					//All the items in the previous rows at the same column are smaller (row->0,col)
					//All the items in the next rows at the same column are smaller (row->maxRowLength-1,col)
                int currentItem = treesCollection[row][col];

                //default to true, if anything is greater than or equal height, we are not visible
                bool visibleFromLeft = true;
                bool visibleFromRight = true;
                bool visibleFromTop = true;
            	bool visibleFromBottom = true;

                //check left side
                for(int tempCol = col-1; tempCol >= 0; tempCol--)
                {
                    const int leftSideItem = treesCollection[row][tempCol];
                    if(leftSideItem >= currentItem)
                    {
                        visibleFromLeft = false;
                        break;
                    }
                }

                //we only need to check the right side if we are not visible on the left
                if(!visibleFromLeft)
                {
                    //check right side
                    for (int tempCol = col + 1; tempCol < maxColLength; tempCol++)
                    {
                        const int  rightSideItem = treesCollection[row][tempCol];
                        if (rightSideItem >= currentItem)
                        {
                            visibleFromRight = false;
                            break;
                        }
                    }
                }

                //we only need to check the top side if we are not visible on the left or right
                if(!(visibleFromLeft || visibleFromRight))
                {
                    //check top side
                    for (int tempRow = row - 1; tempRow >= 0; tempRow--)
                    {
                        const int topSideItem = treesCollection[tempRow][col];
                        if (topSideItem >= currentItem)
                        {
                            visibleFromTop = false;
                            break;
                        }
                    }
                }
                
                //we only need to check the top side if we are not visible on any other side
                if(!(visibleFromLeft || visibleFromRight || visibleFromTop))
                {
                    //check bottom side
                    for (int tempRow = row + 1; tempRow < maxRowLength; tempRow++)
                    {
                        const int bottomSideItem = treesCollection[tempRow][col];
                        if (bottomSideItem >= currentItem)
                        {
                            visibleFromBottom = false;
                            break;
                        }
                    }
                }
              

                if(visibleFromTop || visibleFromBottom || visibleFromLeft || visibleFromRight)
                {
                    visibleTreeCount++;
                }
            }
        }

        cout << "The count of visible trees is " + to_string(visibleTreeCount)+"\n";


        for (int row = 0; row < maxRowLength; row++)
        {
            auto& rowCollection = treesCollection[row];
            const int maxColLength = static_cast<int>(rowCollection.size());
            for (int col = 0; col < maxColLength; col++)
            {
                //items at row 0 have at least one scenic score of 0
                //so we ignore them
                if (row == 0 || row == maxRowLength - 1)
                {
                    continue;
                }

                //items in col 0 have at least one scenic score of 0
                if (col == 0 || col == maxColLength - 1)
                {
                    continue;
                }

                //an items scenic score is
                int currentItem = treesCollection[row][col];

                //default to true, if anything is greater than or equal height, we are not visible
                int leftScore = 0;
                int rightScore = 0;
                int topScore = 0;
                int bottomScore = 0;

                //check left side
                for (int tempCol = col - 1; tempCol >= 0; tempCol--)
                {
                    const int leftSideItem = treesCollection[row][tempCol];
                    leftScore++;
                    if(leftSideItem >= currentItem) //if we run into a lower height tree, we can keep looking
                    {
                        break;
                    }
                }

                //check right side
                for (int tempCol = col + 1; tempCol < maxColLength; tempCol++)
                {
                    const int  rightSideItem = treesCollection[row][tempCol];
                    rightScore++;
                    if(rightSideItem >= currentItem)
                    {
                        break;
                    }
                }

                //check top side
                for (int tempRow = row - 1; tempRow >= 0; tempRow--)
                {
                    const int topSideItem = treesCollection[tempRow][col];
                    topScore++;
                    if(topSideItem >= currentItem)
                    {
                        break;
                    }
                }

                //check bottom side
                for (int tempRow = row + 1; tempRow < maxRowLength; tempRow++)
                {
                    const int bottomSideItem = treesCollection[tempRow][col];
                    bottomScore++;
                    if(bottomSideItem >= currentItem)
                    {
                        break;
                    }
                }

                size_t currentScore = leftScore * rightScore * topScore * bottomScore;
                if(currentScore > highestScenicScore)
                {
                    highestScenicScore = currentScore;
                }

            }
        }

        cout << "The highestScenicScore is " + to_string(highestScenicScore) + "\n";
    }
}

void Problem9()
{
    ifstream inputStream(R"(Problem9Input.txt)");
    if (inputStream.is_open())
    {
        string inputLine;
        RopeSolver solver{};
        RopeSolver longSolver{8};
        while (inputStream)
        {
            std::getline(inputStream, inputLine);

            if (inputLine.empty())
            {
                break;
            }


            const auto breakPos = inputLine.find(' ');
            string inputDir = inputLine.substr(0, 1);
            const int inputDist = stoi(inputLine.substr(breakPos + 1));

            if(inputDir == "R")
            {
                solver.MoveHeadRight(inputDist);
                longSolver.MoveHeadRight(inputDist);
            }
            else if(inputDir == "L")
            {
                solver.MoveHeadLeft(inputDist);
                longSolver.MoveHeadLeft(inputDist);
            }
            else if(inputDir == "U")
            {
                solver.MoveHeadUp(inputDist);
                longSolver.MoveHeadUp(inputDist);
            }
            else if(inputDir == "D")
            {
                solver.MoveHeadDown(inputDist);
                longSolver.MoveHeadDown(inputDist);
            }

            inputLine = "";
        }
        cout << "Tail visited " + to_string(solver.GetTailVisitsCount()) + " unique nodes\n";
        cout << "With 8 midsections, Tail visited " + to_string(longSolver.GetTailVisitsCount()) + " unique nodes\n";
    }
}

int CheckAndUpdateSignalStrength(int cycleCounter, int xRegisterAmount)
{
    if (cycleCounter == 20
        || cycleCounter == 60
        || cycleCounter == 100
        || cycleCounter == 140
        || cycleCounter == 180
        || cycleCounter == 220)
    {
        return (cycleCounter * xRegisterAmount);
    }
    return 0;
}

void Problem10()
{
    ifstream inputStream(R"(Problem10Input.txt)");
    if (inputStream.is_open())
    {
        string inputLine;
        int cycleCounter = 1;
        int xRegisterAmount = 1;
        int signalStrength = 0;
        
        int cpuCol = 0;
        while(inputStream)
        {
            std::getline(inputStream, inputLine);

            if (inputLine.empty())
            {
                break;
            }

            const auto breakPos = inputLine.find(' ');
            if(breakPos == string::npos)
            {
                //no space, we are working with a 'noop' string
                if (cycleCounter == 41
                    || cycleCounter == 81
                    || cycleCounter == 121
                    || cycleCounter == 161
                    || cycleCounter == 201)
                {
                    cout << "\n";
                    cpuCol = 0;
                }
                
                if(xRegisterAmount == cpuCol || xRegisterAmount-1 == cpuCol || xRegisterAmount+1 == cpuCol)
                {
                    cout << "#";
                }
                else
                {
                    cout << ".";
                }
                cycleCounter++;
                cpuCol++;

                const int strengthAtCycle = CheckAndUpdateSignalStrength(cycleCounter, xRegisterAmount);
                if (strengthAtCycle > 0)
                {
                    signalStrength += strengthAtCycle;
                }
            }
            else
            {
                for(int i=0; i < 2; i++)
                {
                    if (cycleCounter == 41
                        || cycleCounter == 81
                        || cycleCounter == 121
                        || cycleCounter == 161
                        || cycleCounter == 201)
                    {
                        cout << "\n";
                        cpuCol = 0;
                    }
                    
                    if (xRegisterAmount == cpuCol || xRegisterAmount - 1 == cpuCol || xRegisterAmount + 1 == cpuCol)
                    {
                        cout << "#";
                    }
                    else
                    {
                        cout << ".";
                    }

                    if (i == 1)
                    {
                        const int addAmt = stoi(inputLine.substr(breakPos + 1));
                        xRegisterAmount += addAmt;
                    }
                    cycleCounter++;
                    cpuCol++;

                    const int strengthAtCycle = CheckAndUpdateSignalStrength(cycleCounter, xRegisterAmount);
                    if (strengthAtCycle > 0)
                    {
                        signalStrength += strengthAtCycle;
                    }
                }
              
            }

            inputLine = "";
        }
        cout << "Signal strength is " + to_string(signalStrength) + "\n";
    }
}

void Problem11()
{
    ifstream inputStream(R"(Problem11Input.txt)");
    if (inputStream.is_open())
    {
        string inputLine;
        MonkeyBusinessSimulator simulator{};
        while (inputStream)
        {
            list<unsigned long long int> worryItems = {};
            operation op;
            unsigned long long int operationFactor = 0;
            unsigned long long int testDivisibleFactor = 0;
            int trueTarget = 0;
            int falseTarget = 0;
            do
            {
                std::getline(inputStream, inputLine);
                //monkey number declaration, continue
                if(inputLine.starts_with("Monkey"))
                {
	                continue;
                }

                size_t labelBreakPos = inputLine.find(':');
                if(labelBreakPos != string::npos)
                {
					//we want to skip the space after the colon
                    string valueSubString = inputLine.substr(labelBreakPos + 2);
                	//items are a comma separated collection of ints
                    if(isdigit(valueSubString[0]))
                    {
                        string currValue;
                        for(size_t charIdx = 0; charIdx < valueSubString.size(); charIdx++)
                        {
	                        if(isdigit(valueSubString[charIdx]))
	                        {
                                currValue += valueSubString[charIdx];
	                        }
                            else if(valueSubString[charIdx] == ',')
                            {
                                unsigned long long int value = stoull(currValue);
                                worryItems.push_back(value);
                                currValue = "";
                            }
                        }

                        if(!currValue.empty())
                        {
                            unsigned long long int value = stoull(currValue);
                            worryItems.push_back(value);
                            currValue = "";
                        }
                    }
                    //get operation and factor
                    else if(valueSubString.starts_with("new"))
                    {
                        string operationFactorStr;
                        size_t multOperatorPos = valueSubString.find('*');
                        size_t addOperatorPos = valueSubString.find('+');
	                    if(multOperatorPos != string::npos)
	                    {
                            op = multiply;
                            operationFactorStr = valueSubString.substr(multOperatorPos + 2);
	                    }
                        else if(addOperatorPos != string::npos)
                        {
                            op = add;
                            operationFactorStr = valueSubString.substr(addOperatorPos + 2);
                        }
                        if(isdigit(operationFactorStr[0]))
                        {
                            operationFactor = stoull(operationFactorStr);
                        }
                        else
                        {
                            operationFactor = SIZE_MAX;
                        }
                    }
                    //get test divisible
                    else if(valueSubString.starts_with("divisible"))
                    {
                        size_t byPosition = valueSubString.find("by");
                        string divisibleStr = valueSubString.substr(byPosition + 3);
                        testDivisibleFactor = stoull(divisibleStr);
                    }
                    //get true target
                    else if(inputLine.starts_with("    If true"))
                    {
                        trueTarget = stoi(valueSubString.substr(valueSubString.size() - 1));
                    }
                    //get true target
                    else if (inputLine.starts_with("    If false"))
                    {
                        falseTarget = stoi(valueSubString.substr(valueSubString.size() - 1));
                    }
                }

            } while (!inputLine.empty() && inputStream);

            inputLine = "";
            simulator.AddMonkey(worryItems, op, operationFactor, testDivisibleFactor, trueTarget, falseTarget);
        }

        while(simulator.GetRoundCounter() < 20)
        {
            simulator.DoRound();
        }
        unsigned long long int monkeyBusinessLevel = simulator.GetMonkeyBusinessLevel();

        cout << "Monkey business level is "+to_string(monkeyBusinessLevel)+"\n";
    }
}

void Problem12()
{
    ifstream inputStream(R"(Problem12Input.txt)");
    if (inputStream.is_open())
    {
        IndexMap indexMap{};
        PathTree pathTree{};
        string inputLine;
        int rowIndex = 0;
        map<char, int> valueMap = {
            {'a',1},
			{'b',2}, {'c',3},
            {'d',4}, {'e',5},
            {'f',6}, {'g',7},
            {'h',8}, {'i',9},
            {'j',10}, {'k',11},
            {'l',12}, {'m',13},
            {'n',14}, {'o',15},
            {'p',16}, {'q',17},
            {'r',18}, {'s',19},
            {'t',20}, {'u',21},
        	{'v',22},{'w',23},
            {'x',24},{'y',25},
            {'z',26},

        	{'E',27},
            {'S',0},

        };

        while (inputStream)
        {
            std::getline(inputStream, inputLine);

            if (inputLine.empty())
            {
                break;
            }

            for (size_t col = 0; col < inputLine.length(); col++)
            {
                char& inputChar = inputLine[col];
                int inputInt = valueMap[inputChar];
                indexMap.AddIndexItem(rowIndex, col, inputInt);
            }

            inputLine = "";
            rowIndex++;
        }

        pathTree.BuildTree(indexMap);
        pathTree.PrintTree();
        int shortTestPath = pathTree.TraverseShortestPathToGoal();

    }
}

int main()
{
    Problem6();
    Problem6(false);
    cout << "\n";
    Problem7();
    cout << "\n";
    Problem8();
    cout << "\n";
    Problem9();
    cout << "\n";
    Problem10();
    cout << "\n";
    Problem11();
    cout << "\n";
    Problem12();
}
