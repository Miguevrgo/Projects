//
// Created by miguevr on 5/17/24.
//

#ifndef RIDMAZE_TEXTUI_H
#define RIDMAZE_TEXTUI_H


#include "UI.h"
#include <iostream>
#include <string>

class TextUI : public UI {
public:
    TextUI() = default;
    auto nextMove() -> Directions override;
    void showGame(const GameState& gameState) override;

private:
    static auto readChar() -> char;
};


#endif //RIDMAZE_TEXTUI_H
