//
// Created by miguevr on 4/25/24.
//

#include "Dice.h"

bool Dice::breakWeapon() {
    std::random_device rd;
    std::mt19937 gen(rd());
    std::uniform_real_distribution<double> dis(0, 1);
    return dis(gen) < BREAK_PROB;
}

double Dice::randomAttack() {
    std::random_device rd;
    std::mt19937 gen(rd());
    std::uniform_real_distribution<double> dis(0, MAX_ATTACK);
    return dis(gen);
}
