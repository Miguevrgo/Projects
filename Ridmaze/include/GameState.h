//
// Created by miguevr on 5/17/24.
//

#ifndef RIDMAZE_GAMESTATE_H
#define RIDMAZE_GAMESTATE_H

#include <string_view>
#include <string>

class GameState {
public:
    GameState(std::string_view labyrinth, std::string_view players, std::string_view monsters, int currentPlayer, bool winner, std::string_view log);

    [[nodiscard]] auto getLabyrinth() const -> std::string;
    [[nodiscard]] auto getPlayers() const -> std::string;
    [[nodiscard]] auto getMonsters() const -> std::string;
    [[nodiscard]] auto getCurrentPlayer() const -> int;
    [[nodiscard]] auto isWinner() const -> bool;
    [[nodiscard]] auto getLog() const -> std::string;

private:
    std::string labyrinth;
    std::string players;
    std::string monsters;
    int currentPlayer;
    bool winner;
    std::string log;
};



#endif //RIDMAZE_GAMESTATE_H
