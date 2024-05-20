//
// Created by miguevr on 5/17/24.
//

#include "Labyrinth.h"
#include "Dice.h"
#include <sstream>

Labyrinth::Labyrinth(int nRows, int nCols, int exitRow, int exitCol)
        : nRows(nRows), nCols(nCols), exitRow(exitRow), exitCol(exitCol),
          labyrinth(nRows, std::vector<char>(nCols, EMPTY_CHAR)),
          monsters(nRows, std::vector<std::shared_ptr<Monster>>(nCols, nullptr)),
          players(nRows, std::vector<std::shared_ptr<Player>>(nCols, nullptr)) {
    labyrinth[exitRow][exitCol] = EXIT_CHAR;
}

void Labyrinth::spreadPlayers(const std::vector<std::shared_ptr<Player>>& players) {
    for (auto& player : players) {
        auto [row, col] = randomEmptyPos();
        putPlayer2D(INVALID_POS, INVALID_POS, row, col, player);
    }
}

auto Labyrinth::haveAWinner() const -> bool {
    return players[exitRow][exitCol] != nullptr;
}

auto Labyrinth::toString() const -> std::string {
    std::ostringstream oss;
    for (int i = 0; i < nRows; ++i) {
        for (int j = 0; j < nCols; ++j) {
            oss << labyrinth[i][j];
        }
    }
    return oss.str();
}

void Labyrinth::addMonster(int row, int col, std::shared_ptr<Monster> monster) {
    if (posOK(row, col) && emptyPos(row, col)) {
        labyrinth[row][col] = MONSTER_CHAR;
        monsters[row][col] = monster;
        monster->setPos(row, col);
    }
}

auto Labyrinth::putPlayer(Directions direction, std::shared_ptr<Player> player) -> std::shared_ptr<Monster> {
    auto [oldRow, oldCol] = std::make_tuple(player->getRow(), player->getCol());
    auto [newRow, newCol] = dir2Pos(oldRow, oldCol, direction);
    return putPlayer2D(oldRow, oldCol, newRow, newCol, player);
}

void Labyrinth::addBlock(Orientation orientation, int startRow, int startCol, int length) {
    int incRow = (orientation == Orientation::VERTICAL) ? 1 : 0;
    int incCol = (orientation == Orientation::HORIZONTAL) ? 1 : 0;

    for (int row = startRow, col = startCol; posOK(row, col) && emptyPos(row, col) && length > 0; row += incRow, col += incCol, --length) {
        labyrinth[row][col] = BLOCK_CHAR;
    }
}

auto Labyrinth::validMoves(int row, int col) const -> std::vector<Directions> {
    std::vector<Directions> moves;
    if (canStepOn(row - 1, col)) moves.push_back(Directions::UP);
    if (canStepOn(row + 1, col)) moves.push_back(Directions::DOWN);
    if (canStepOn(row, col - 1)) moves.push_back(Directions::LEFT);
    if (canStepOn(row, col + 1)) moves.push_back(Directions::RIGHT);
    return moves;
}

void Labyrinth::updatePos(const std::shared_ptr<Player>& player) {
    players[player->getRow()][player->getCol()] = player;
}

auto Labyrinth::posOK(int row, int col) const -> bool {
    return row >= 0 && row < nRows && col >= 0 && col < nCols;
}

auto Labyrinth::emptyPos(int row, int col) const -> bool {
    return labyrinth[row][col] == EMPTY_CHAR;
}

auto Labyrinth::monsterPos(int row, int col) const -> bool {
    return labyrinth[row][col] == MONSTER_CHAR;
}

auto Labyrinth::exitPos(int row, int col) const -> bool {
    return labyrinth[row][col] == EXIT_CHAR;
}

auto Labyrinth::combatPos(int row, int col) const -> bool {
    return labyrinth[row][col] == COMBAT_CHAR;
}

auto Labyrinth::canStepOn(int row, int col) const -> bool {
    return posOK(row, col) && (emptyPos(row, col) || monsterPos(row, col) || exitPos(row, col));
}

void Labyrinth::updateOldPos(int row, int col) {
    if (posOK(row, col)) {
        if (combatPos(row, col)) {
            labyrinth[row][col] = MONSTER_CHAR;
        } else {
            labyrinth[row][col] = EMPTY_CHAR;
        }
    }
}

auto Labyrinth::dir2Pos(int row, int col, Directions direction) const -> std::tuple<int, int> {
    switch (direction) {
        case Directions::UP:
            row -= 1;
            break;
        case Directions::DOWN:
            row += 1;
            break;
        case Directions::LEFT:
            col -= 1;
            break;
        case Directions::RIGHT:
            col += 1;
            break;
    }
    return std::make_tuple(row, col);
}

auto Labyrinth::randomEmptyPos() const -> std::tuple<int, int> {
    int row, col;
    do {
        row = Dice::randomPos(nRows);
        col = Dice::randomPos(nCols);
    } while (!emptyPos(row, col));
    return std::make_tuple(row, col);
}

auto Labyrinth::putPlayer2D(int oldRow, int oldCol, int row, int col, std::shared_ptr<Player> player) -> std::shared_ptr<Monster> {
    std::shared_ptr<Monster> output = nullptr;

    if (canStepOn(row, col)) {
        if (posOK(oldRow, oldCol) && players[oldRow][oldCol] == player) {
            updateOldPos(oldRow, oldCol);
            players[oldRow][oldCol] = nullptr;
        }

        if (monsterPos(row, col)) {
            labyrinth[row][col] = COMBAT_CHAR;
            output = monsters[row][col];
        } else {
            labyrinth[row][col] = player->getNumber();
        }

        players[row][col] = player;
        player->setPos(row, col);
    }

    return output;
}