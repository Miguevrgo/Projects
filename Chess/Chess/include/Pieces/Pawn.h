#pragma once
#include "Piece.h"

class Pawn : public Piece{
public:
    bool makeMove(Coordinates start, Coordinates end);
};

