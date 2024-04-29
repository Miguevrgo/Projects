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

double Dice::randomStrength() {
    std::random_device rd;
    std::mt19937 gen(rd());
    std::uniform_real_distribution<double> dis(0, MAX_STRENGTH);
    return dis(gen);
}

int Dice::randomRarity() {
    std::random_device rd;
    std::mt19937 gen(rd());
    std::uniform_int_distribution<int> dis(0, 3);
    return dis(gen);
}
