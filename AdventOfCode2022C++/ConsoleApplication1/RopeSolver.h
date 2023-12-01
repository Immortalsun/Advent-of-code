#pragma once
#include <map>
#include <unordered_set>
using namespace std;
class Point
{
	private:
		int X = 0;
		int Y = 0;

	public:
		Point() = default;

		Point(int x, int y)
		{
			X = x;
			Y = y;
		}


		bool operator==(const Point& p) const
		{
			if(this->X == p.X && this->Y == p.Y)
			{
				return true;
			}
			return false;
		}

		int GetX()
		{
			return X;
		}

		int GetY()
		{
			return Y;
		}

		void SetX(int newX)
		{
			X = newX;
		}

		void SetY(int newY)
		{
			Y = newY;
		}
};

struct PointHashFunction
{
	public:
		size_t operator()(const Point& p) const
		{
			auto xValue = static_cast<Point>(p).GetX();
			auto yValue = static_cast<Point>(p).GetY();
			return (xValue * 130) + (yValue * 130);
		}
};

class Rope
{
	private:
		Point _head;
		map<int, Point> _midSectionMap;
		Point _tail;

	public:
		Rope(int midsectionCount = 0)
		{
			_midSectionMap = {};
			if(midsectionCount > 0)
			{
				for(int i=0; i<midsectionCount; i++)
				{
					_midSectionMap[i] = Point();
				}
			}
		}

		map<int, Point>& GetMidsections()
		{
			return _midSectionMap;
		}

		void MoveHeadPosition(int newX, int newY)
		{
			_head.SetX(newX);
			_head.SetY(newY);
		}

		void MoveMidSectionPosition(int midSectionIndex, int newX, int newY)
		{
			auto& midSection = _midSectionMap[midSectionIndex];
			midSection.SetX(newX);
			midSection.SetY(newY);
		}

		void MoveTailPosition(int newX, int newY)
		{
			_tail.SetX(newX);
			_tail.SetY(newY);
		}

		Point GetMidSectionPosition(int midSectionIndex)
		{
			auto& midSection = _midSectionMap[midSectionIndex];
			return {midSection.GetX(), midSection.GetY()};
		}

		Point GetHeadPosition()
		{
			return {_head.GetX(), _head.GetY()};
		}

		Point GetTailPosition()
		{
			return {_tail.GetX(), _tail.GetY()};
		}
};

class RopeSolver
{
	private:
		Rope _rope;
		int _midSectionCount;
		unordered_set<Point, PointHashFunction> _tailVisits;

	public:
		RopeSolver(int midSectionCount = 0) : _rope(midSectionCount)
		{
			_midSectionCount = midSectionCount;
			_tailVisits.insert(_rope.GetTailPosition());
		}

		void MoveHeadLeft(int leftDist)
		{
			for(int i=0; i<leftDist; i++)
			{
				auto currentHeadPos = _rope.GetHeadPosition();
				auto newHeadX = currentHeadPos.GetX() - 1;
				_rope.MoveHeadPosition(newHeadX, currentHeadPos.GetY());
				if(_midSectionCount == 0)
				{
					CheckAndMoveTail("L");
				}
				else
				{
					CheckAndMoveMidPoints("L");
				}
			}
		}

		void MoveHeadRight(int rightDist)
		{
			for(int i=0; i<rightDist; i++)
			{
				auto currentHeadPos = _rope.GetHeadPosition();
				auto newHeadX = currentHeadPos.GetX() + 1;
				_rope.MoveHeadPosition(newHeadX, currentHeadPos.GetY());
				if (_midSectionCount == 0)
				{
					CheckAndMoveTail("R");
				}
				else
				{
					CheckAndMoveMidPoints("R");
				}
			}
		}

		void MoveHeadUp(int upDist)
		{
			for(int i=0; i<upDist; i++)
			{
				auto currentHeadPos = _rope.GetHeadPosition();
				auto newHeadY = currentHeadPos.GetY() + 1;
				_rope.MoveHeadPosition(currentHeadPos.GetX(), newHeadY);
				if (_midSectionCount == 0)
				{
					CheckAndMoveTail("U");
				}
				else
				{
					CheckAndMoveMidPoints("U");
				}
			}
		}

		void MoveHeadDown(int downDist)
		{
			for(int i=0; i<downDist; i++)
			{
				auto currentHeadPos = _rope.GetHeadPosition();
				auto newHeadY = currentHeadPos.GetY() - 1;
				_rope.MoveHeadPosition(currentHeadPos.GetX(), newHeadY);
				if (_midSectionCount == 0)
				{
					CheckAndMoveTail("D");
				}
				else
				{
					CheckAndMoveMidPoints("D");
				}
			}
		}

		Point MovePointBasedOnPreviousPosition(string dir, Point currentPoint, Point previousPoint)
		{
			int newCurrentX = currentPoint.GetX();
			int newCurrentY = currentPoint.GetY();
			//previous and current are in the same position
			if (previousPoint.GetX() == currentPoint.GetX() && previousPoint.GetY() == currentPoint.GetY())
			{
				//do nothing
				return Point(newCurrentX, newCurrentY);
			}

			//previous and current are within 1 position on both x and y
			if(abs(previousPoint.GetX() - currentPoint.GetX()) <= 1 && abs(previousPoint.GetY() - currentPoint.GetY()) <= 1)
			{
				//do nothing
				return Point(newCurrentX, newCurrentY);
			}


			auto xDifference = previousPoint.GetX() - currentPoint.GetX();
			auto yDifference = previousPoint.GetY() - currentPoint.GetY();
			if (xDifference > 0)
			{
				newCurrentX = currentPoint.GetX() + 1;
			}
			else if (xDifference < 0)
			{
				newCurrentX = currentPoint.GetX() - 1;
			}

			if (yDifference > 0)
			{
				newCurrentY = currentPoint.GetY() + 1;
			}
			else if (yDifference < 0)
			{
				newCurrentY = currentPoint.GetY() - 1;
			}

			return Point{ newCurrentX, newCurrentY };
		}

		void CheckAndMoveTail(string dir)
		{

			auto newtailPos = MovePointBasedOnPreviousPosition(dir, 
				_rope.GetTailPosition(), _rope.GetHeadPosition());

			_rope.MoveTailPosition(newtailPos.GetX() , newtailPos.GetY());

			_tailVisits.insert(Point(newtailPos.GetX(), newtailPos.GetY()));
		}

		void CheckAndMoveMidPoints(string dir)
		{
			//move midpoints
			for(int pointIndex = 0; pointIndex < _midSectionCount; pointIndex++)
			{
				Point prevPointPosition;
				const Point currentPointPosition = _rope.GetMidSectionPosition(pointIndex);

				if (pointIndex == 0)
				{
					prevPointPosition = _rope.GetHeadPosition();
				}
				else
				{
					prevPointPosition = _rope.GetMidSectionPosition(pointIndex - 1);
				}

				auto newCurrentPosition = MovePointBasedOnPreviousPosition(dir, currentPointPosition, prevPointPosition);

				_rope.MoveMidSectionPosition(pointIndex, newCurrentPosition.GetX(), newCurrentPosition.GetY());
			}

			//move tail
			auto newtailPos = MovePointBasedOnPreviousPosition(dir, _rope.GetTailPosition(),
				_rope.GetMidSectionPosition(_midSectionCount - 1));

			_rope.MoveTailPosition(newtailPos.GetX(), newtailPos.GetY());

			_tailVisits.insert(Point(newtailPos.GetX(), newtailPos.GetY()));
		}

		size_t GetTailVisitsCount() const
		{
			return _tailVisits.size();
		}
};