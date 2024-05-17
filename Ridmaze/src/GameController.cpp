//
// Created by miguevr on 5/17/24.
//
#include "GameController.h"
#include <iostream>

GameController::GameController(int nPlayers)
    : game(nPlayers), ui(std::make_unique<TextUI>()) {}

void GameController::run() {
    while (!game.finished()) {
        update();
        render();
        handlePlayerInput();
    }
    std::cout << "The game has ended!" << std::endl;
    render();
}

void GameController::update() {
    // Aquí se puede actualizar la lógica del juego si es necesario
}

void GameController::render() {
    GameState state = game.getGameState();
    ui->showGame(state);
}

void GameController::handlePlayerInput() {
    Directions direction = ui->nextMove();
    bool gameEnded = game.nextStep(direction);

    if (gameEnded) {
        std::cout << "The game has ended!" << std::endl;
    }
}
