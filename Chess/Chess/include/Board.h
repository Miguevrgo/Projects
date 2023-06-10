#pragma once
#include "Pieces/Pieces.h"
#include "SFML/Graphics.hpp"
#include "Pieces/Bishop.h"
#include "Pieces/Rook.h"
#include "Pieces/Knight.h"
#include "Pieces/King.h"
#include "Pieces/Queen.h"
#include "Pieces/Pawn.h"

struct Squares{
	Pieces* piece;
	sf::Sprite sprite;
};

class Board{
public:
	Board();
	void initializeBoard();
private:
	static const unsigned short int DIM = 8;
	Squares _board[DIM][DIM];
};

