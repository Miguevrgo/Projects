#pragma once
#include <string>
#include "Player.h"

struct Coordinates {
	unsigned int row;
	unsigned int col;
};
// Name of class should be changed to Piece and reorganize all
class Pieces{
public:
	bool initPiece(int id, Player owner, Coordinates pos);
	Pieces* getPiece(Coordinates pos);
	virtual bool makeMove(Coordinates start, Coordinates end);
	virtual void run();
	virtual bool gameOver() const = 0;
protected:
	static unsigned const int SIZE = 8;
	unsigned int turn;
	Pieces* matrixPieces[SIZE][SIZE];
	Pieces* newPiece(int id, Player owner);
private:
	void initializeMatrix();
};

