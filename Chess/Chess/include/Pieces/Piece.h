#pragma once
#include <string>
#include "Player.h"

struct Coordinates {
	unsigned int row;
	unsigned int col;
};

class Piece {
public:
	bool initPiece(int id, Player owner, Coordinates pos);
	Piece* getPiece(Coordinates pos);
	virtual bool makeMove(Coordinates start, Coordinates end);
	virtual void run();
	virtual bool gameOver() const = 0;
protected:
	static unsigned const int SIZE = 8;
	unsigned int turn;
	Piece* matrixPieces[SIZE][SIZE];
	Piece* newPiece(int id, Player owner);
private:
	void initializeMatrix();
};

