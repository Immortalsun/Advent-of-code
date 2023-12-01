#pragma once
#include <map>
#include "IndexMap.h";
using namespace std;

class PathNode
{
	private:
		bool _isStart;
		bool _isGoal;
		bool _hasLeftChild = false;
		bool _hasRightChild = false;
		bool _hasUpChild = false;
		bool _hasDownChild = false;
		int _heightValue;
		PathNode* _upChild;
		PathNode* _downChild;
		PathNode* _rightChild;
		PathNode* _leftChild;

	public:
		PathNode()
		{
			_isStart = true;
			_isGoal = false;
			_heightValue = 1;
		}

		PathNode(bool isStart, bool isGoal, int heightValue)
		{
			_isStart = isStart;
			_isGoal = isGoal;
			_heightValue = heightValue;
		}

		void AddUpChild(PathNode& nodeChild)
		{
			_upChild = &nodeChild;
			_hasUpChild = true;
		}

		void AddDownChild(PathNode& nodeChild)
		{
			_downChild = &nodeChild;
			_hasDownChild = true;
		}

		void AddLeftChild(PathNode& nodeChild)
		{
			_leftChild = &nodeChild;
			_hasLeftChild = true;
		}

		void AddRightChild(PathNode& nodeChild)
		{
			_rightChild = &nodeChild;
			_hasRightChild = true;
		}

		int GetValue()
		{
			return _heightValue;
		}

		bool HasUpChild()
		{
			return _hasUpChild;
		}

		bool HasLeftChild()
		{
			return _hasLeftChild;
		}

		bool HasRightChild()
		{
			return _hasRightChild;
		}

		bool HasDownChild()
		{
			return _hasDownChild;
		}

		PathNode& GetLeftChild()
		{
			return *_leftChild;
		}

		PathNode& GetRightChild()
		{
			return *_rightChild;
		}

		PathNode& GetUpChild()
		{
			return *_upChild;
		}

		PathNode& GetDownChld()
		{
			return *_downChild;
		}
};

class PathTree
{
	private:
		PathNode _root = PathNode(true, false, 1);
		PathNode* _currentNode;
		map<int, map<int, PathNode>> _pathTree;

	public:
		PathTree()
		{
			_currentNode = &_root;
			_pathTree = {};
		}

		void BuildTree(IndexMap indexMap)
		{
			map<int, map<int, int>> indexItems = indexMap.GetIndexItems();
			for(int row = 0; row < indexItems.size(); row++)
			{
				auto& currentRow = indexItems[row];
				for(int column = 0; column < currentRow.size(); column++)
				{
					bool isStart = currentRow[column] == 0;
					bool isEnd = currentRow[column] == 27;
					int value = currentRow[column];

					//start and end of abnormal values in initial collection
					//to identify them
					if(isStart)
					{
						value = 1;
					}
					if(isEnd)
					{
						value = 26;
					}

					PathNode newNode = PathNode(isStart, isEnd, value);

					//reset root when we find the start
					if (isStart)
					{
						_root = newNode;
						_currentNode = &_root;
					}

					if (_pathTree.contains(row))
					{
						auto& pathRow = _pathTree[row];
						pathRow[column] = newNode;
					}
					else
					{
						map<int, PathNode> newPathRow = {};
						newPathRow[column] = newNode;
						_pathTree[row] = newPathRow;
					}

					int upChildRow = row - 1;
					int upChildCol = column;

					int leftChildRow = row;
					int leftChildCol = column - 1;

					if(upChildRow >= 0)
					{
					   auto& upChild = _pathTree[upChildRow][upChildCol];
					   newNode.AddUpChild(upChild);
					   upChild.AddDownChild(newNode);
					   _pathTree[row][column] = newNode;
					}

					if(leftChildCol >= 0)
					{
						auto& leftChild = _pathTree[leftChildRow][leftChildCol];
						newNode.AddLeftChild(leftChild);
						leftChild.AddRightChild(newNode);
						_pathTree[row][column] = newNode;
					}
				}
			}
		}

		int TraverseShortestPathToGoal()
		{
			return 0;
		}

		void PrintTree()
		{
			for(int row = 0; row < _pathTree.size(); row++)
			{
				auto& currentTreeRow = _pathTree[row];
				for(int col = 0; col < currentTreeRow.size(); col++)
				{
					string nodeOutputString;
					auto& currNode = currentTreeRow[col];
					if(currNode.HasLeftChild())
					{
						nodeOutputString += "<";
					}
					if (currNode.HasDownChild())
					{
						nodeOutputString += "v";
					}
					nodeOutputString += to_string(currNode.GetValue());
					if(currNode.HasUpChild())
					{
						nodeOutputString += "^";
					}
					if(currNode.HasRightChild())
					{
						nodeOutputString += ">";
					}
					cout << nodeOutputString;
				}
				cout << endl;
			}
		}
};