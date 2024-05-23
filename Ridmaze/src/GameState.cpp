//
// Created by miguevr on 5/17/24.
//

#include "GameState.h"

GameState::GameState(std::string_view labyrinth, std::string_view player, std::string_view monsters, bool winner, std::string_view log)
        : labyrinth(labyrinth), player(player), monsters(monsters), winner(winner), log(log) {}

auto GameState::getLabyrinth() const -> std::string {
    return labyrinth;
}

auto GameState::getPlayer() const -> std::string {
    return player;
}

auto GameState::getMonsters() const -> std::string {
    return monsters;
}

auto GameState::getCurrentPlayer() const -> int {
    return currentPlayer;
}

auto GameState::isWinner() const -> bool {
    return winner;
}

auto GameState::getLog() const -> std::string {
    return log;
}

