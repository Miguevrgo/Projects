//
// Created by miguevr on 5/24/24.
//

#include <sstream>
#include "Level.h"

Level::Level(const std::string_view& configFile) : stairRow(-1), stairCol(-1), inputFile(configFile) {
    configureLevel();
}

void Level::configureLevel() {
    std::ifstream input(inputFile);

    std::string size;
    std::getline(input, size);
    std::istringstream iss(size);
    int nRows, nCols;
    iss >> nRows >> nCols;

    level.resize(nRows, std::vector<char>(nCols, EMPTY_CHAR));
    monsters.resize(nRows, std::vector<std::shared_ptr<Monster>>(nCols, nullptr));

    std::string line;
    int row = 0;

    while (std::getline(input, line) && row < level.size()) {
        for (int col = 0; col < line.size() && col < nCols; ++col) {
            char cell = line[col];

            switch (cell) {
                case BLOCK_CHAR:
                case EMPTY_CHAR:
                case EXIT_CHAR:
                case STAIRCASE_CHAR:
                    level[row][col] = cell;
                    if (cell == STAIRCASE_CHAR){
                        stairRow = row;
                        stairCol = col;
                    }
                case MONSTER_CHAR: {
                    auto monster = std::make_shared<Monster>("Monster", Dice::randomIntelligence(), Dice::randomStrength());
                    monsters[row][col] = monster;
                    level[row][col] = MONSTER_CHAR;
                }
                default:
                    level[row][col] = EMPTY_CHAR;
            }
        }

        ++row;
    }
}

std::string Level::toString() const {
    std::ostringstream oss;
    for (const auto& row : level) {
        for (char cell : row) {
            oss << cell;
        }
        oss << '\n';
    }

    return oss.str();
}

char Level::getCell(int row, int col) const {
    if (row >= 0 && row < level.size() && col >= 0 && col < level[row].size()) {
        return level[row][col];
    }

    return EMPTY_CHAR;
}

int Level::getStairRow() const {
    return stairRow;
}

int Level::getStairCol() const {
    return stairCol;
}

std::shared_ptr<Monster> Level::getMonster(int row, int col) const {
    if (row >= 0 && row < level.size() && col >= 0 && col < level[row].size()) {
        return monsters[row][col];
    }

    return nullptr;
}

void Level::setCell(int row, int col, char content) {
    level[row][col] = content;
}

int Level::getCols(int row) const {
    return level[row].size();
}

int Level::getRows() const {
    return level.size();
}

