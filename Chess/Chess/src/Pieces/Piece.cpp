#include "Pieces/Piece.h"

bool Piece::initPiece(int id, Player owner, Coordinates pos){
    
}

Piece* Piece::getPiece(Coordinates pos){
    return matrixPieces[pos.row][pos.col];
}

bool Piece::makeMove(Coordinates start, Coordinates end){

}

void Piece::run(){

}

bool Piece::gameOver() const{

}

void Piece::initializeMatrix(){

}
