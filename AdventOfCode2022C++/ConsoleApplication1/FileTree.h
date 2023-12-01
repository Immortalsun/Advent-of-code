#pragma once
#include <iostream>
#include <string>
#include <list>
using namespace std;

class FileNode
{
	private:
		string _name;
		size_t _size = 0;

	public:
		FileNode(string name, size_t size)
		{
			_name = name;
			_size = size;
		}

		string GetName()
		{
			return _name;
		}

		int GetSize()
		{
			return _size;
		}
};

class DirectoryNode
{
	private:
		string _name;
		DirectoryNode* _parent;
		list<FileNode> _files;
		list<DirectoryNode> _subDirectories;

	public:
		DirectoryNode(string name)
		{
			_name = name;
			_files = {};
			_subDirectories = {};
		}

		void AddFileChild(FileNode& item)
		{
			_files.push_back(item);
		}

		void AddDirectoryChild(DirectoryNode& item)
		{
			item._parent = this;
			_subDirectories.push_back(item);
		}

		DirectoryNode& GetChildDirectoryWithName(string childName)
		{
			for (auto& child : _subDirectories)
			{
				if (child.GetName() == childName)
				{
					return child;
				}
			}

			throw std::exception("node not found");
		}

		DirectoryNode* GetParent()
		{
			return _parent;
		}

		list<DirectoryNode>& GetSubDirectories()
		{
			return _subDirectories;
		}


		list<FileNode>& GetFiles()
		{
			return _files;
		}

		string GetName()
		{
			return _name;
		}
		
};


class FileTree
{
	private:
		DirectoryNode _root = DirectoryNode("/");
		DirectoryNode* _currentNode;

	public:
		FileTree()
		{
			_currentNode = &_root;
		}

		void AddFileChildToCurrentNode(FileNode& node)
		{
			DirectoryNode& curr = *_currentNode;
			curr.AddFileChild(node);
		}

		void AddDirectoryChildToCurrentNode(DirectoryNode& node)
		{
			DirectoryNode& curr = *_currentNode;
			curr.AddDirectoryChild(node);
		}

		void NavigateCurrentToParent()
		{
			DirectoryNode& curr = *_currentNode;
			_currentNode = curr.GetParent();
		}

		void ChangeCurrentToNode(string nodeName)
		{
			//short circuit root navigation, other nodes may have cycles of the same name
			//but there is only one root
			if(_currentNode->GetName() == "/" && nodeName == "/")
			{
				return;
			}

			auto& child = _currentNode->GetChildDirectoryWithName(nodeName);

			_currentNode = &child;
		}

		DirectoryNode& GetRoot()
		{
			return _root;
		}

		string GetCurrentNodeName()
		{
			return _currentNode->GetName();
		}
};

class TreeTraverser
{
	public:
		size_t totalDirectorySize = 0;
		size_t sizeUpperBound = 100000;
		size_t smallestSizeOverLimit = 43956976;

		//total space is: 70,000,000
		//root dir size is: 43,956,976
		//free space is: 26,043,024
		//desired space is: 30,000,000
		//space needed to be freed is at least: 3,956,976

		size_t GetDirectorySize(DirectoryNode& directory)
		{
			size_t totalOutputSize = 0;

			for (auto& element : directory.GetSubDirectories())
			{
				totalOutputSize += GetDirectorySize(element);
			}

			for (auto& fileElement : directory.GetFiles())
			{
				totalOutputSize += fileElement.GetSize();
			}

			return totalOutputSize;
		}

		void GetTotalDirectorySizesUnderLimit(DirectoryNode& directory)
		{
			size_t currentDirectorySize = GetDirectorySize(directory);
			if (currentDirectorySize <= sizeUpperBound)
			{
				totalDirectorySize += currentDirectorySize;
			}

			for (auto& item : directory.GetSubDirectories())
			{
				GetTotalDirectorySizesUnderLimit(item);
			}
		}

		void GetSmallestDirectorOverSizeLimit(DirectoryNode& directory, size_t limit = 3956976)
		{
			size_t currentDirectorySize = GetDirectorySize(directory);
			if(currentDirectorySize >= limit && currentDirectorySize < smallestSizeOverLimit)
			{
				cout << "Directory " + directory.GetName() + "is below bound and of size " + std::to_string(currentDirectorySize) + "\n";
				smallestSizeOverLimit = currentDirectorySize;
			}

			for (auto& item : directory.GetSubDirectories())
			{
				GetSmallestDirectorOverSizeLimit(item);
			}
		}

		void Reset()
		{
			totalDirectorySize = 0;
		}
};
