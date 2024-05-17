//
// Created by miguevr on 5/17/24.
//

#ifndef RIDMAZE_GAMECONTROLLER_H
#define RIDMAZE_GAMECONTROLLER_H


#include "Game.h"
#include "TextUI.h"
#include <SFML/Graphics.hpp>
#include <memory>

class GameController {
public:
    GameController(int nPlayers);
    void run();

private:
    Game game;
    sf::RenderWindow window;
    sf::Font font;
    std::unique_ptr<TextUI> ui;

    void processEvents();
    void update();
    void render();
    void handlePlayerInput(sf::Keyboard::Key key);
    void drawGameState(const GameState& state);
};


#endif //RIDMAZE_GAMECONTROLLER_H
