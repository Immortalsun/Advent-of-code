#pragma once
#include <map>
#include <list>
#include <queue>
using namespace std;

enum operation{multiply=0, add};

class Monkey
{
	private:
		queue<unsigned long long int> _itemWorryLevels;
		operation _op;
		unsigned long long int _inspectionCount = 0;
		unsigned long long int _operationFactor;
		unsigned long long int _testDivisibleFactor;
		int _trueTargetMonkeyIndex;
		int _falseTargetMonkeyIndex;
		
	public:
		Monkey()
		{
			_itemWorryLevels = {};
			_op = multiply;
			_operationFactor = 0;
			_testDivisibleFactor = 0;
			_trueTargetMonkeyIndex = 0;
			_falseTargetMonkeyIndex = 0;
		}
		
		Monkey(list<unsigned long long int> itemWorryLevels, operation op, unsigned long long int operationFactor, unsigned long long int testDivisibleFactor, int trueTarget, int falseTarget )
		{
			for (size_t element : itemWorryLevels)
			{
				_itemWorryLevels.push(element);
			}
			_op = op;
			_operationFactor = operationFactor;
			_testDivisibleFactor = testDivisibleFactor;
			_trueTargetMonkeyIndex = trueTarget;
			_falseTargetMonkeyIndex = falseTarget;
		}

		unsigned long long int GetOperationFactor()
		{
			return _operationFactor;
		}

		unsigned long long int GetAndPopNextItemWorryLevel()
		{
			unsigned long long int itemLevel =_itemWorryLevels.front();
			_itemWorryLevels.pop();
			return itemLevel;
		}

		operation GetOperation()
		{
			return _op;
		}

		bool HasWorryLevelItems()
		{
			return !_itemWorryLevels.empty();
		}

		void AddItemWorryLevel(unsigned long long int worryLevel)
		{
			_itemWorryLevels.push(worryLevel);
		}

		unsigned long long int GetTestDivisbleFactor()
		{
			return _testDivisibleFactor;
		}

		int GetTrueMonkeyTargetIndex()
		{
			return _trueTargetMonkeyIndex;
		}

		int GetFalstMonkeyTargetIndex()
		{
			return _falseTargetMonkeyIndex;
		}

		unsigned long long int GetInspectionCount()
		{
			return _inspectionCount;
		}

		void IncreaseInspectionCount()
		{
			_inspectionCount += 1;
		}

		void PrintItems(int monkeyIdx)
		{
			cout << "Monkey " + to_string(monkeyIdx) + " has items: ";
			size_t size = _itemWorryLevels.size();
			for(size_t i=0; i < size; i++)
			{
				unsigned long long int worryLevel = GetAndPopNextItemWorryLevel();
				cout << to_string(worryLevel)+" ";
				AddItemWorryLevel(worryLevel);
			}
			cout << "\n";
		}
};

class MonkeyBusinessSimulator
{
	private:
		map<int, Monkey> _monkeys;
		int _monkeyCount = 0;
		int _roundCounter;

	public:
		MonkeyBusinessSimulator()
		{
			_monkeys = {};
			_roundCounter = 0;
		}

		void AddMonkey(list<unsigned long long int> itemWorryLevels, operation op, unsigned long long int operationFactor, unsigned long long int testDivisibleFactor, int trueTarget, int falseTarget)
		{
			auto newMonkey = Monkey(itemWorryLevels, op, operationFactor, testDivisibleFactor, trueTarget, falseTarget);
			_monkeys[_monkeyCount] = newMonkey;
			_monkeyCount++;
		}

		void DoRound()
		{
			for(int i=0; i < _monkeys.size(); i++)
			{
				auto& currMonkey = _monkeys[i];

				//if monkey has no items to inspect, move to next
				while (currMonkey.HasWorryLevelItems())
				{
					unsigned long long int oldWorryLevel = currMonkey.GetAndPopNextItemWorryLevel();

					unsigned long long int operationFactor = currMonkey.GetOperationFactor();

					//operation factor is max unsigned int if the operation factor is the worry level itself
					if (operationFactor  == SIZE_MAX)
					{
						operationFactor = oldWorryLevel;
					}

					unsigned long long int newWorryLevel = 0;

					if (currMonkey.GetOperation() == multiply)
					{
						newWorryLevel = oldWorryLevel * operationFactor;
					}
					else if (currMonkey.GetOperation() == add)
					{
						newWorryLevel = oldWorryLevel + operationFactor;
					}

					//money gets bored, divide by 3
					newWorryLevel = newWorryLevel / 3;

					unsigned long long int divisibleFactor = currMonkey.GetTestDivisbleFactor();
	
					if(newWorryLevel % divisibleFactor == 0)
					{
						auto& trueTargetMonkey = _monkeys[currMonkey.GetTrueMonkeyTargetIndex()];
						trueTargetMonkey.AddItemWorryLevel(newWorryLevel);
					}
					else
					{
						auto& falseTargetMonkey = _monkeys[currMonkey.GetFalstMonkeyTargetIndex()];
						falseTargetMonkey.AddItemWorryLevel(newWorryLevel);
					}

					currMonkey.IncreaseInspectionCount();
				}
			}
			_roundCounter++;

			cout << "After Round " + to_string(_roundCounter) + "\n";
			for(int i = 0; i < _monkeys.size(); i++)
			{
				
				_monkeys[i].PrintItems(i);
			}
			cout << endl;
		}

		unsigned long long int GetMonkeyBusinessLevel()
		{
			unsigned long long int highestInspections = 0;
			unsigned long long int secondHighestInspections = 0;
			for (int i=0; i < _monkeys.size(); i++)
			{
				auto& currMonkey = _monkeys[i];
				if(currMonkey.GetInspectionCount() > highestInspections)
				{
					int prevHighestInspections = highestInspections;
					highestInspections = currMonkey.GetInspectionCount();
					secondHighestInspections = prevHighestInspections;
				}
				else if(currMonkey.GetInspectionCount() > secondHighestInspections)
				{
					secondHighestInspections = currMonkey.GetInspectionCount();
				}
			}
			return highestInspections * secondHighestInspections;
		}

		int GetRoundCounter()
		{
			return _roundCounter;
		}
};