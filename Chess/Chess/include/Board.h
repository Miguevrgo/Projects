#pragma once
#include "SFML/Graphics.hpp"
#include "Pieces/Pieces.h"
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
	/**
	 * @brief Initializes the board setting the images of each square
	 * 
	 */
	void initializeBoard();
	/**
	 * @brief Get the Piece object in the provided location
	 * 
	 * @param coordinates row and column
	 * @return Pointer to the piece at the given location 
	 */
	Pieces* getPiece(Coordinates coordinates);
private:
	static const unsigned short int DIM = 8;
	Squares _board[DIM][DIM];
};

