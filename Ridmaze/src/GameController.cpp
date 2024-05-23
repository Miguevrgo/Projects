#include "GameController.h"
#include <iostream>

GameController::GameController(int rows, int cols)
        : game(rows, cols) {}

void GameController::handlePlayerInput(sf::Keyboard::Key key, sf::Window& window) {
    Directions direction;
    bool validInput = true;

    switch (key) {
        case sf::Keyboard::Up:
        case sf::Keyboard::K:
            direction = Directions::UP;
            break;
        case sf::Keyboard::Down:
        case sf::Keyboard::J:
            direction = Directions::DOWN;
            break;
        case sf::Keyboard::Left:
        case sf::Keyboard::H:
            direction = Directions::LEFT;
            break;
        case sf::Keyboard::Right:
        case sf::Keyboard::L:
            direction = Directions::RIGHT;
            break;
        default:
            validInput = false;
            break;
    }

    if (validInput) {
        bool gameEnded = game.nextStep(direction);
        if (gameEnded) {
            std::cout << "The game has ended!" << std::endl;
            window.close();
        }
    }
}

Game GameController::getGame() const {
    return game;
}


