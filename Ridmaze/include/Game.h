#ifndef RIDMAZE_GAME_H
#define RIDMAZE_GAME_H

#include <memory>
#include <string>
#include <vector>
#include "Labyrinth.h"
#include "Player.h"
#include "Monster.h"
#include "Directions.h"
#include "GameCharacter.h"
#include "GameState.h"


class Game {
public:
    Game(const std::vector<std::string_view>& configFiles);

    [[nodiscard]] bool finished() const;
    bool nextStep(Directions preferredDirection);
    [[nodiscard]] GameState getGameState() const;

private:
    static const int MAX_ROUNDS = 10;

    std::shared_ptr<Player> player;
    std::vector<std::shared_ptr<Monster>> monsters;
    Labyrinth labyrinth;
    std::string log;

    auto actualDirection(Directions preferredDirection) -> Directions;
    auto combat(const std::shared_ptr<Monster>& monster) -> GameCharacter;
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

#endif // RIDMAZE_GAME_H
