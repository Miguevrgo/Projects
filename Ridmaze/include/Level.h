//
// Created by miguevr on 5/24/24.
//

#ifndef RIDMAZE_LEVEL_H
#define RIDMAZE_LEVEL_H

#include <string>
#include <vector>
#include <sstream>
#include <fstream>
#include <memory>
#include "Monster.h"

class Level {
public:
    Level(const std::string_view& configFile);
    [[nodiscard]] std::string toString() const;
    [[nodiscard]] char getCell(int row, int col) const;
    void setCell(int row, int col, char content);
    [[nodiscard]] std::shared_ptr<Monster> getMonster(int row, int col) const;
    [[nodiscard]] int getStairRow() const;
    [[nodiscard]] int getStairCol() const;
    [[nodiscard]] int getCols(int row) const;
    [[nodiscard]] int getRows() const;

private:
    /**
     * Configures current level using input from configFile provided to the object stored in inputFile, with the
     * following format:
     *
     * [nRows] [nCols]
     * [Level Cells]
     */
    void configureLevel();

    std::vector<std::vector<char>> level;
    std::vector<std::vector<std::shared_ptr<Monster>>> monsters;
    std::string inputFile;

    int stairRow;
    int stairCol;

    static constexpr char BLOCK_CHAR = 'X';
    static constexpr char EMPTY_CHAR = '-';
    static constexpr char MONSTER_CHAR = 'M';
    static constexpr char UP_STAIRCASE_CHAR = 'U';
    static constexpr char DOWN_STAIRCASE_CHAR = 'D';
    static constexpr char EXIT_CHAR = 'E';
};


#endif //RIDMAZE_LEVEL_H
