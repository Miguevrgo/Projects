#pragma once
#include <string>

struct Coordinates {
	unsigned int row;
	char col;
};

class Pieces{
public:
	Pieces(const std::string colour);
	~Pieces();
	Coordinates getCoordinates();
	void setCoordinates(Coordinates coords);
	/**
	 * @brief Checks if given move is valid. Checks if king is or would be in 
	 * check and if the move is contained in piece valid moves
	 * 
	 * @return true if the given move is valid
	 * @return false if the given move cannot be performed
	 */
	bool isValidMovement(Coordinates move); // Deciding between isValidMovement or getValidMovements
	/**
	 * @brief Returns if the given movement is a check
	 * 
	 * @return true 
	 * @return false 
	 */
	bool isCheck(Coordinates move);
	/**
	 * @brief Changes the current piece coronated to the provided one 
	 * (Queen,Rook,Knight or Bishop)
	 * 
	 * @param new_piece 
	 */
	void changePiece(Pieces new_piece);
private: 
	const std::string colour;
};

