#include "Labyrinth.h"
#include "Dice.h"
#include <sstream>

Labyrinth::Labyrinth(const std::vector<std::string_view>& levelConfigFiles) {
    labyrinth.reserve(levelConfigFiles.size());
    for (auto configFile : levelConfigFiles) {
        labyrinth.emplace_back(configFile);
    }

    currentLevel = 0;
}

void Labyrinth::placePlayer(const std::shared_ptr<Player>& player) {
    this->player = player;
    movePlayer2D(INVALID_POS, INVALID_POS, 0, 0);
}

auto Labyrinth::haveAWinner() const -> bool {
    return player && labyrinth[currentLevel].getCell(player->getRow(),player->getCol()) == EXIT_CHAR;
}

auto Labyrinth::toString() const -> std::string {
    std::ostringstream oss;
    oss << labyrinth[currentLevel].toString();

    return oss.str();
}

auto Labyrinth::movePlayer(Directions direction) -> std::shared_ptr<Monster> {
    auto [oldRow, oldCol] = std::make_tuple(player->getRow(), player->getCol());
    auto [newRow, newCol] = dir2Pos(oldRow, oldCol, direction);
    return movePlayer2D(oldRow, oldCol, newRow, newCol);
}

void Labyrinth::addBlock(Orientation orientation, int startRow, int startCol, int length) {
    int incRow = (orientation == Orientation::VERTICAL) ? 1 : 0;
    int incCol = (orientation == Orientation::HORIZONTAL) ? 1 : 0;

    for (int row = startRow, col = startCol; posOK(row, col) && emptyPos(row, col) && length > 0; row += incRow, col += incCol, --length) {
        labyrinth[currentLevel].setCell(row, col, BLOCK_CHAR);
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

auto Labyrinth::posOK(int row, int col) const -> bool {
    return row >= 0 && row < labyrinth[currentLevel].getRows() && col >= 0 && col < labyrinth[currentLevel].getCols(row);
}

auto Labyrinth::emptyPos(int row, int col) const -> bool {
    return labyrinth[currentLevel].getCell(row, col) == EMPTY_CHAR;
}

auto Labyrinth::monsterPos(int row, int col) const -> bool {
    return labyrinth[currentLevel].getCell(row, col) == MONSTER_CHAR;
}

auto Labyrinth::exitPos(int row, int col) const -> bool {
    return labyrinth[currentLevel].getCell(row, col) == EXIT_CHAR;
}

auto Labyrinth::combatPos(int row, int col) const -> bool {
    return labyrinth[currentLevel].getCell(row, col) == COMBAT_CHAR;
}

auto Labyrinth::canStepOn(int row, int col) const -> bool {
    return posOK(row, col) && (emptyPos(row, col) || monsterPos(row, col) || exitPos(row, col) || stairPos(row, col));
}

void Labyrinth::updateOldPos(int row, int col) {
    if (posOK(row, col)) {
        if (combatPos(row, col)) {
            labyrinth[currentLevel].setCell(row, col, MONSTER_CHAR);
        }
        else if (stairPos(row, col)) {
            labyrinth[currentLevel].setCell(row, col, STAIRCASE_CHAR);
        }
        else {
            labyrinth[currentLevel].setCell(row, col, EMPTY_CHAR);
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

auto Labyrinth::movePlayer2D(int oldRow, int oldCol, int row, int col) -> std::shared_ptr<Monster> {
    std::shared_ptr<Monster> output = nullptr;

    if (canStepOn(row, col)) {
        if (posOK(oldRow, oldCol)) {
            updateOldPos(oldRow, oldCol);
        }

        if (monsterPos(row, col)) {
            labyrinth[currentLevel].setCell(row, col, COMBAT_CHAR);
            output = labyrinth[currentLevel].getMonster(row,col);
        }
        else if (stairPos(row, col)) {
            currentLevel++;
            row = labyrinth[currentLevel].getStairRow();
            col = labyrinth[currentLevel].getStairCol();
        }
        else if (exitPos(row, col)) {
            labyrinth[currentLevel].setCell(row, col, EXIT_CHAR);
        }
        else {
            labyrinth[currentLevel].setCell(row, col, PLAYER_CHAR);
        }

        player->setPos(row, col);
    }

    return output;
}

auto Labyrinth::stairPos(int row, int col) const -> bool {
    return labyrinth[currentLevel].getCell(row, col) == STAIRCASE_CHAR;
}

