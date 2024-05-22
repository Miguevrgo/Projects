//
// Created by miguevr on 5/17/24.
//

#include "GameState.h"

GameState::GameState(std::string_view labyrinth, std::string_view players, std::string_view monsters, int currentPlayer, bool winner, std::string_view log)
        : labyrinth(labyrinth), players(players), monsters(monsters), currentPlayer(currentPlayer), winner(winner), log(log) {}

auto GameState::getLabyrinth() const -> std::string {
    return labyrinth;
}

auto GameState::getPlayers() const -> std::string {
    return players;
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

