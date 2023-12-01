#pragma once
#include <map>

using namespace std;
class IndexMap
{
	private:
		map<int, map<int, int>> _indexItems;

	public:
		IndexMap()
		{
			_indexItems = {};
		}

		void AddIndexItem(int row, int column, int value)
		{
			if(_indexItems.contains(row))
			{
				auto& colMap = _indexItems[row];
				colMap[column] = value;
			}
			else
			{
				map<int, int> newMap = {};
				newMap[column] = value;
				_indexItems[row] = newMap;
			}
		}

		map<int, map<int,int>>& GetIndexItems()
		{
			return _indexItems;
		}
};