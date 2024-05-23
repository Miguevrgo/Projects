//
// Created by miguevr on 5/17/24.
//

#ifndef RIDMAZE_LABYRINTH_H
#define RIDMAZE_LABYRINTH_H

#include "Player.h"
#include "Monster.h"
#include "Orientation.h"
#include "Directions.h"
#include <vector>
#include <string>
#include <memory>
#include <tuple>

class Labyrinth {
public:
    Labyrinth(int nRows, int nCols, int exitRow, int exitCol);

    void placePlayer(const std::shared_ptr<Player>& player);
    [[nodiscard]] auto haveAWinner() const -> bool;
    [[nodiscard]] auto toString() const -> std::string;
    void addMonster(int row, int col, std::shared_ptr<Monster> monster);
    auto movePlayer(Directions direction) -> std::shared_ptr<Monster>;
    void addBlock(Orientation orientation, int startRow, int startCol, int length);
    void addStaircase(int row, int col);
    [[nodiscard]] auto isOnStaircase() const -> bool;
    [[nodiscard]] auto validMoves(int row, int col) const -> std::vector<Directions>;
    [[nodiscard]] auto getRows() const -> int;
    [[nodiscard]] auto getCols() const -> int;

private:
    static constexpr char BLOCK_CHAR = 'X';
    static constexpr char EMPTY_CHAR = '-';
    static constexpr char MONSTER_CHAR = 'M';
    static constexpr char PLAYER_CHAR = 'P';
    static constexpr char COMBAT_CHAR = 'C';
    static constexpr char EXIT_CHAR = 'E';
    static constexpr char STAIRCASE_CHAR = 'S';
    static constexpr int INVALID_POS = -1;

    int nRows;
    int nCols;
    int exitRow;
    int exitCol;

    std::vector<std::vector<char>> labyrinth;
    std::vector<std::vector<std::shared_ptr<Monster>>> monsters;
    std::shared_ptr<Player> player;

    [[nodiscard]] auto posOK(int row, int col) const -> bool;
    [[nodiscard]] auto emptyPos(int row, int col) const -> bool;
    [[nodiscard]] auto monsterPos(int row, int col) const -> bool;
    [[nodiscard]] auto exitPos(int row, int col) const -> bool;
    [[nodiscard]] auto combatPos(int row, int col) const -> bool;
    [[nodiscard]] auto canStepOn(int row, int col) const -> bool;
    void updateOldPos(int row, int col);
    [[nodiscard]] auto dir2Pos(int row, int col, Directions direction) const -> std::tuple<int, int>;
    [[nodiscard]] auto randomEmptyPos() const -> std::tuple<int, int>;
    auto movePlayer2D(int oldRow, int oldCol, int row, int col) -> std::shared_ptr<Monster>;
};

#endif //RIDMAZE_LABYRINTH_H
