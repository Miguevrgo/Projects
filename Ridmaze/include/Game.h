//
// Created by miguevr on 5/17/24.
//

#ifndef RIDMAZE_GAME_H
#define RIDMAZE_GAME_H

#include <vector>
#include <string>
#include <string_view>
#include <memory>
#include "Labyrinth.h"
#include "Player.h"
#include "Monster.h"
#include "GameCharacter.h"
#include "GameState.h"
#include "Directions.h"
#include "Orientation.h"

class Game {
public:
    Game(int nPlayers);

    [[nodiscard]] auto finished() const -> bool;
    auto nextStep(Directions preferredDirection) -> bool;
    [[nodiscard]] auto getGameState() const -> GameState;

private:
    static const int MAX_ROUNDS = 10;
    static const int ROWS = 11;
    static const int COLS = 10;
    static const int NMONSTERS = 5;
    static const int MONSTERS_POS[NMONSTERS][2];
    static const int NBLOCKS = 4;
    static const std::tuple<Orientation, int, int, int> BLOCKS_POS[NBLOCKS];

    int currentPlayerIndex;
    std::shared_ptr<Player> currentPlayer;
    std::string log;
    Labyrinth labyrinth;
    std::vector<std::shared_ptr<Player>> players;
    std::vector<std::shared_ptr<Monster>> monsters;

    void configureLabyrinth();
    void nextPlayer();
    auto actualDirection(Directions preferredDirection) -> Directions;
    auto combat(std::shared_ptr<Monster> monster) -> GameCharacter;
    void manageReward(GameCharacter winner);
    void manageResurrection();

    void logPlayerWon();
    void logMonsterWon();
    void logResurrected();
    void logPlayerSkipTurn();
    void logPlayerNoOrders();
    void logNoMonster();
    void logRounds(int rounds, int max);
};


#endif //RIDMAZE_GAME_H
