/**
 * @file Cursor.cpp
 * @author Miguel Ángel De la Vega Rodríguez
 * @brief This file contains the implementation for the methods in Cursor class
 */

#include "Cursor.h"

Cursor::Cursor() : Cursor(0,0) {}

Cursor::Cursor(int lineNumber, int charNumber) : lineNumber(lineNumber), charNumber(charNumber) {}

void Cursor::MoveUp(int numLines) {
    lineNumber += numLines;
}

void Cursor::MoveDown(int numLines) {
    lineNumber -= numLines;
}

void Cursor::MoveRight(int numChars) {
    charNumber += numChars;
}

void Cursor::MoveLeft(int numChars) {
    charNumber -= numChars;
}

int Cursor::GetLineNumber() const {
    return lineNumber;
}

int Cursor::GetCharNumber() const {
    return charNumber;
}

