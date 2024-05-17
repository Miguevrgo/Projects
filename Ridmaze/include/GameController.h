//
// Created by miguevr on 5/17/24.
//

#ifndef RIDMAZE_GAMECONTROLLER_H
#define RIDMAZE_GAMECONTROLLER_H

#include "Game.h"
#include "TextUI.h"
#include <memory>

class GameController {
public:
    GameController(int nPlayers);
    void run();

private:
    Game game;
    std::unique_ptr<TextUI> ui;

    void handlePlayerInput();
    void update();
    void render();
};


#endif //RIDMAZE_GAMECONTROLLER_H
