using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

namespace AdventOfCode2022
{
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("--- Problem 1 Output---");
            Problem1();
            Console.WriteLine("");

            Console.WriteLine("--- Problem 2 Output---");
            Problem2();
            Console.WriteLine("");

            Console.WriteLine("--- Problem 3 Output---");
            Problem3();
            Console.WriteLine("");

            Console.WriteLine("--- Problem 4 Output---");
            Problem4();
            Console.WriteLine("");

            Console.WriteLine("--- Problem 5 Output---");
            Problem5();
            Problem5(false);
            Console.WriteLine("");
        }

        private static void Problem1()
        {
            int currentCalorieValue = 0;
            int maxCaloriesValue = 0;
            int rank2CaloriesValue = 0;
            int rank3CaloriesValue = 0;
            using (var reader = new StreamReader(File.OpenRead(Environment.CurrentDirectory+"/ProblemInput/Problem1Input.txt")))
            {
                while (!reader.EndOfStream)
                {
                    var line = reader.ReadLine();
                    if (!string.IsNullOrEmpty(line) && int.TryParse(line, out var calories))
                    {
                        currentCalorieValue += calories;
                    }
                    else
                    {
                        
                        if (currentCalorieValue > maxCaloriesValue)
                        {
                            var prevMaxValue = maxCaloriesValue;
                            maxCaloriesValue = currentCalorieValue;

                            var prevRank2Value = rank2CaloriesValue;
                            rank2CaloriesValue = prevMaxValue;

                            rank3CaloriesValue = prevRank2Value;
                        }
                        else if (currentCalorieValue > rank2CaloriesValue)
                        {
                            var prevRank2Value = rank2CaloriesValue;
                            rank2CaloriesValue = currentCalorieValue;

                            rank3CaloriesValue = prevRank2Value;
                        }
                        else if (currentCalorieValue > rank3CaloriesValue)
                        {
                            rank3CaloriesValue = currentCalorieValue;
                        }

                        currentCalorieValue = 0;
                    }
                }
            }

            Console.WriteLine($"Max calories among elves is {maxCaloriesValue}");
            Console.WriteLine($"Rank 2 calories among elves is {rank2CaloriesValue}");
            Console.WriteLine($"Rank 3 calories among elves is {rank3CaloriesValue}");
            Console.WriteLine($"Total Top 3 calories among elves is {maxCaloriesValue+rank2CaloriesValue+rank3CaloriesValue}");
        }

        private static void Problem2()
        {
            //rock paper scissors 
            //part 1 opponent: A = Rock, B = Paper, C = Scissors
            //part 1 self: X = Rock, Y = Paper, Z = Scissors
            //Scores for player choice: Rock = 1pt, Paper = 2pt, Scissors = 3pt
            //Scores for Result: win = 6pt, draw = 3pt, loss = 0pt
            //Total Score for round = playChoice + match result

            //part 1: score if strategy guide is adhered to strictly
            int totalScore = 0;
            int currentRoundScore = 0;
            using (var reader =
                   new StreamReader(File.OpenRead(Environment.CurrentDirectory + "/ProblemInput/Problem2Input.txt")))
            {
                while (!reader.EndOfStream)
                {
                    var line = reader.ReadLine();
                    if (string.IsNullOrEmpty(line))
                    {
                        break;
                    }

                    var choices = line.Split(' ', StringSplitOptions.RemoveEmptyEntries);
                    
                    string opponentChoice = choices[0].ToLowerInvariant();
                    string playerChoice = choices[1].ToLowerInvariant();
                    switch (opponentChoice)
                    {
                        case "a":
                            switch (playerChoice)
                            {
                                case "x":
                                    currentRoundScore = 4; //draw(3) + rock played(1)
                                    break;
                                case "y":
                                    currentRoundScore = 8; //win(6) + paper played(2)
                                    break;
                                case "z":
                                    currentRoundScore = 3; //loss(0) + scissors played(3)
                                    break;
                            }
                            break;
                        case "b":
                            switch (playerChoice)
                            {
                                case "x":
                                    currentRoundScore = 1; //loss(0) + rock played(1)
                                    break;
                                case "y":
                                    currentRoundScore = 5; //draw(3) + paper played(2)
                                    break;
                                case "z":
                                    currentRoundScore = 9; //win(6) + scissors played(3)
                                    break;
                            }
                            break;
                        case "c":
                            switch (playerChoice)
                            {
                                case "x":
                                    currentRoundScore = 7; //win(6) + rock played(1)
                                    break;
                                case "y":
                                    currentRoundScore = 2; //loss(0) + paper played(2)
                                    break;
                                case "z":
                                    currentRoundScore = 6; //draw(3) + scissors played(3)
                                    break;
                            }
                            break;
                    }

                    totalScore += currentRoundScore;
                    currentRoundScore = 0;
                }
            }

            Console.WriteLine($"Total Score for Part 1 Guide is {totalScore}");

            //part 2: score if second column is result of round where X=lose, y=draw, z=win
            totalScore = 0;
            currentRoundScore = 0;
            using (var reader =
                  new StreamReader(File.OpenRead(Environment.CurrentDirectory + "/ProblemInput/Problem2Input.txt")))
            {
                while (!reader.EndOfStream)
                {
                    var line = reader.ReadLine();
                    if (string.IsNullOrEmpty(line))
                    {
                        break;
                    }

                    var entries = line.Split(' ', StringSplitOptions.RemoveEmptyEntries);

                    string opponentChoice = entries[0].ToLowerInvariant();
                    string roundResult = entries[1].ToLowerInvariant();
                    switch (opponentChoice)
                    {
                        case "a":
                            switch (roundResult)
                            {
                                case "x":
                                    currentRoundScore = 3; //loss(0) + scissors played(3)
                                    break;
                                case "y":
                                    currentRoundScore = 4; //draw(3) + rock played(1)
                                    break;
                                case "z":
                                    currentRoundScore = 8; //win(6) + paper played(2)
                                    break;
                            }
                            break;
                        case "b":
                            switch (roundResult)
                            {
                                case "x":
                                    currentRoundScore = 1; //loss(0) + rock played(1)
                                    break;
                                case "y":
                                    currentRoundScore = 5; //draw(3) + paper played(2)
                                    break;
                                case "z":
                                    currentRoundScore = 9; //win(6) + scissors played(3)
                                    break;
                            }
                            break;
                        case "c":
                            switch (roundResult)
                            {
                                case "x":
                                    currentRoundScore = 2; //loss(0) + paper played(2)
                                    break;
                                case "y":
                                    currentRoundScore = 6; //draw(3) + scissors played(3)
                                    break;
                                case "z":
                                    currentRoundScore = 7; //win(6) + rock played(1)
                                    break;
                            }
                            break;
                    }

                    totalScore += currentRoundScore;
                    currentRoundScore = 0;
                }

                Console.WriteLine($"Total Score for Part 2 Guide is {totalScore}");
            }
        }

        private static void Problem3()
        {
            string contentsInput = "";
            string pack1Content = "";
            string pack2Content = "";
            int totalPriority = 0;
            var priorityMap = new Dictionary<char, int>
            {
                { 'a', 1 },
                { 'b', 2 },
                { 'c', 3 },
                { 'd', 4 },
                { 'e', 5 },
                { 'f', 6 },
                { 'g', 7 },
                { 'h', 8 },
                { 'i', 9 },
                { 'j', 10 },
                { 'k', 11 },
                { 'l', 12 },
                { 'm', 13 },
                { 'n', 14 },
                { 'o', 15 },
                { 'p', 16 },
                { 'q', 17 },
                { 'r', 18 },
                { 's', 19 },
                { 't', 20 },
                { 'u', 21 },
                { 'v', 22 },
                { 'w', 23 },
                { 'x', 24 },
                { 'y', 25 },
                { 'z', 26 }
            };
            using (var reader =
                   new StreamReader(File.OpenRead(Environment.CurrentDirectory + "/ProblemInput/Problem3Input.txt")))
            {
                while (!reader.EndOfStream)
                {
                    contentsInput = reader.ReadLine();
                    if (string.IsNullOrEmpty(contentsInput))
                    {
                        throw new InvalidOperationException("Empty string from file");
                    }

                    pack1Content = contentsInput.Substring(0, (contentsInput.Length / 2));
                    pack2Content = contentsInput.Substring(contentsInput.Length / 2);

                    if (pack1Content.Length != pack2Content.Length)
                    {
                        throw new InvalidOperationException("Unequal lengths");
                    }

                    foreach (var item in pack1Content)
                    {
                        char matchingItem = pack2Content.FirstOrDefault(n => n.Equals(item));
                        if (matchingItem.Equals('\0'))
                        {
                            continue;
                        }

                        var isLowerCase = priorityMap.TryGetValue(matchingItem, out int priority);
                        if (isLowerCase)
                        {
                            totalPriority += priority;
                        }
                        else
                        {
                            var lowerCaseMatchingItem = matchingItem.ToString().ToLowerInvariant()[0];
                            priorityMap.TryGetValue(lowerCaseMatchingItem, out int lowerPriority);
                            totalPriority += (lowerPriority + 26);
                        }
                        break;
                    }
                }
            }
            Console.WriteLine($"Part 1 --> Total priorities across all rucksacks is {totalPriority}");


            var groupArray = new string[3];
            contentsInput = "";
            int smallestPackIdx = 0;
            totalPriority = 0;
            using (var reader =
                   new StreamReader(File.OpenRead(Environment.CurrentDirectory + "/ProblemInput/Problem3Input.txt")))
            {
                while (!reader.EndOfStream)
                {
                    int minPackLength = 0;
                    for (int i = 0; i < groupArray.Length; i++)
                    {
                        var packContents = reader.ReadLine();
                        if (string.IsNullOrEmpty(packContents))
                        {
                            throw new InvalidOperationException("Empty input");
                        }

                        groupArray[i] = packContents;
                        if (minPackLength == 0)
                        {
                            minPackLength = packContents.Length;
                        }
                        else if(packContents.Length < minPackLength)
                        {
                            minPackLength = packContents.Length;
                            smallestPackIdx = i;
                        }
                    }

                    contentsInput = groupArray[smallestPackIdx];
                    var otherPacks = groupArray.Where(n => !string.Equals(n,contentsInput)).ToList();
                    string otherPackInput1 = otherPacks.First();
                    string otherpackInput2 = otherPacks.Last();

                    foreach (var item in contentsInput)
                    {
                        char matching1Item = otherPackInput1.FirstOrDefault(n => n.Equals(item));
                        char matching2Item = otherpackInput2.FirstOrDefault(n => n.Equals(item));
                        if (matching1Item.Equals('\0') || matching2Item.Equals('\0') ||
                            !matching1Item.Equals(matching2Item))
                        {
                            continue;
                        }

                        var isLowerCase = priorityMap.TryGetValue(matching1Item, out int priority);
                        if (isLowerCase)
                        {
                            totalPriority += priority;
                        }
                        else
                        {
                            var lowerCaseMatchingItem = matching1Item.ToString().ToLowerInvariant()[0];
                            priorityMap.TryGetValue(lowerCaseMatchingItem, out int lowerPriority);
                            totalPriority += (lowerPriority + 26);
                        }
                        break;
                    }
                }
            }
            Console.WriteLine($"Part 2 --> Total priorities across grouped rucksacks is {totalPriority}");
        }

        private static void Problem4()
        {
            int totalOverlapRangeCount = 0;
            int anyOverlapRangecount = 0;
            using (var reader =
                   new StreamReader(File.OpenRead(Environment.CurrentDirectory + "/ProblemInput/Problem4Input.txt")))
            {
                while (!reader.EndOfStream)
                {
                    var input = reader.ReadLine();
                    if (string.IsNullOrEmpty(input))
                    {
                        throw new InvalidOperationException("Empty input");
                    }

                    var ranges = input.Split(',', StringSplitOptions.RemoveEmptyEntries);
                    var firstRange = ranges[0].Split('-', StringSplitOptions.RemoveEmptyEntries);
                    var secondRange = ranges[1].Split('-', StringSplitOptions.RemoveEmptyEntries);

                    if (int.TryParse(firstRange[0], out int firstRangeLeftBound) &&
                        int.TryParse(firstRange[1], out int firstRangeRightBound) &&
                        int.TryParse(secondRange[0], out int secondRangeLeftBound) &&
                        int.TryParse(secondRange[1], out int secondRangeRightBound))
                    {
                        //first range contains second range
                        if ((firstRangeLeftBound <= secondRangeLeftBound &&
                            firstRangeRightBound >= secondRangeRightBound) ||

                            //second range contains first range
                            (secondRangeLeftBound <= firstRangeLeftBound 
                             && secondRangeRightBound >= firstRangeRightBound))
                        {
                            totalOverlapRangeCount++;
                        }

                        //any parts of the ranges overlap
                        if ((firstRangeRightBound >= secondRangeLeftBound 
                                  && firstRangeRightBound <= secondRangeRightBound) ||

                                 (secondRangeRightBound >= firstRangeLeftBound 
                                  && secondRangeRightBound <= firstRangeRightBound))
                        {
                            anyOverlapRangecount++;
                        }
                    }
                }
            }
            Console.WriteLine($"Part 1 --> Total completely overlapping ranges is {totalOverlapRangeCount}");
            Console.WriteLine($"Part 2 --> Total any portion overlapping ranges is {anyOverlapRangecount}");
        }

        private static void Problem5(bool part1 = true)
        {

            /* Stack arrangement
             *          [G]         [D]     [Q]    
                [P]     [T]         [L] [M] [Z]    
                [Z] [Z] [C]         [Z] [G] [W]    
                [M] [B] [F]         [P] [C] [H] [N]
                [T] [S] [R]     [H] [W] [R] [L] [W]
                [R] [T] [Q] [Z] [R] [S] [Z] [F] [P]
                [C] [N] [H] [R] [N] [H] [D] [J] [Q]
                [N] [D] [M] [G] [Z] [F] [W] [S] [S]
                 1   2   3   4   5   6   7   8   9 
             */
            List<string> stackContents = new List<string>
            {
                "NCRTMZP",
                "DNTSBZ",
                "MHQRFCTG",
                "GRZ",
                "ZNRH",
                "FHSWPZLD",
                "WDZRCGM",
                "SJFLHWZQ",
                "SQPWN"
            };
            List<Stack<char>> stacks = new List<Stack<char>>();
            for (int i = 0; i < stackContents.Count; i++)
            {
                var newStack = new Stack<char>();
                foreach (var stackContent in stackContents[i])
                {
                    newStack.Push(stackContent);
                }
                stacks.Add(newStack);
            }

            if (part1)
            {
                using (var reader =
                       new StreamReader(File.OpenRead(Environment.CurrentDirectory + "/ProblemInput/Problem5Input.txt")))
                {
                    int originStackPosition = 0;
                    int destinationStackPosition = 0;
                    int numberMoves = 0;
                    while (!reader.EndOfStream)
                    {
                        var inputLine = reader.ReadLine();
                        if (string.IsNullOrEmpty(inputLine))
                        {
                            throw new Exception("empty input");
                        }

                        var inputLineArray = inputLine.Split(' ', StringSplitOptions.RemoveEmptyEntries);

                        if (int.TryParse(inputLineArray[1], out numberMoves) &&
                            int.TryParse(inputLineArray[3], out originStackPosition) &&
                            int.TryParse(inputLineArray[5], out destinationStackPosition))
                        {
                            var originStack = stacks[originStackPosition - 1];
                            var destinationStack = stacks[destinationStackPosition - 1];
                            for (int i = 0; i < numberMoves; i++)
                            {
                                var item = originStack.Pop();
                                destinationStack.Push(item);
                            }
                        }
                    }

                    var outputStackTops = "";
                    foreach (var stack in stacks)
                    {
                        outputStackTops += stack.Peek();
                    }

                    Console.WriteLine($"Part 1 --> Items at the top of the stacks expressed as: {outputStackTops}");
                }
            }
            else
            {
                using (var reader =
                       new StreamReader(File.OpenRead(Environment.CurrentDirectory + "/ProblemInput/Problem5Input.txt")))
                {
                    int originStackPosition = 0;
                    int destinationStackPosition = 0;
                    int numberMoves = 0;
                    while (!reader.EndOfStream)
                    {
                        var inputLine = reader.ReadLine();
                        if (string.IsNullOrEmpty(inputLine))
                        {
                            throw new Exception("empty input");
                        }

                        var inputLineArray = inputLine.Split(' ', StringSplitOptions.RemoveEmptyEntries);

                        if (int.TryParse(inputLineArray[1], out numberMoves) &&
                            int.TryParse(inputLineArray[3], out originStackPosition) &&
                            int.TryParse(inputLineArray[5], out destinationStackPosition))
                        {
                            var originStack = stacks[originStackPosition - 1];
                            var destinationStack = stacks[destinationStackPosition - 1];
                            if (numberMoves > 1)
                            {
                                List<char> items = new List<char>();
                                for (int i = 0; i < numberMoves; i++)
                                {
                                    items.Add(originStack.Pop());
                                }

                                items.Reverse();
                                foreach (var item in items)
                                {
                                    destinationStack.Push(item);
                                }
                            }
                            else
                            {
                                var item = originStack.Pop();
                                destinationStack.Push(item);
                            }
                        }
                    }

                    var outputStackTops = "";
                    foreach (var stack in stacks)
                    {
                        outputStackTops += stack.Peek();
                    }

                    Console.WriteLine($"Part 2 --> Items at the top of the stacks expressed as: {outputStackTops}");
                }
            }

        }
    }
}
