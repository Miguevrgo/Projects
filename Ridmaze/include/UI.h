//
// Created by miguevr on 5/17/24.
//

#ifndef RIDMAZE_UI_H
#define RIDMAZE_UI_H


#include "Directions.h"
#include "GameState.h"

class UI {
public:
    virtual ~UI() = default;
    virtual auto nextMove() -> Directions = 0;
    virtual void showGame(const GameState& gameState) = 0;
};


#endif //RIDMAZE_UI_H
