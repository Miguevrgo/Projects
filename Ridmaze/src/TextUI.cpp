//
// Created by miguevr on 5/17/24.
//

#include "TextUI.h"
#include <iostream>
#include <string>

auto TextUI::readChar() -> char {
    std::string s;
    std::getline(std::cin, s);
    return s.empty() ? '\0' : s[0];
}

auto TextUI::nextMove() -> Directions {
    std::cout << "Where? ";

    Directions direction = Directions::DOWN;
    bool gotInput = false;

    while (!gotInput) {
        char c = readChar();
        switch (c) {
            case 'w':
                std::cout << " UP\n";
                direction = Directions::UP;
                gotInput = true;
                break;
            case 's':
                std::cout << " DOWN\n";
                direction = Directions::DOWN;
                gotInput = true;
                break;
            case 'd':
                std::cout << " RIGHT\n";
                direction = Directions::RIGHT;
                gotInput = true;
                break;
            case 'a':
                std::cout << " LEFT\n";
                direction = Directions::LEFT;
                gotInput = true;
                break;
            default:
                std::cout << "Invalid input. Please enter 'w', 'a', 's', or 'd'.\n";
                break;
        }
    }
    return direction;
}

void TextUI::showGame(const GameState& gameState) {
    std::cout << gameState.getLabyrinth() << std::endl;
    std::cout << "Players: \n" << gameState.getPlayers() << std::endl;
    std::cout << "Monsters: \n" << gameState.getMonsters() << std::endl;
    std::cout << "Log:\n" << gameState.getLog() << std::endl;

    if (gameState.isWinner()) {
        std::cout << "Player " << gameState.getCurrentPlayer() << " has won! Congratulations \n";
    } else {
        std::cout << "Current player: " << gameState.getCurrentPlayer() << "\n";
    }
}