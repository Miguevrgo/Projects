//
// Created by miguevr on 4/25/24.
//

#ifndef RIDMAZE_DICE_H
#define RIDMAZE_DICE_H

#include <random>

class Dice {
private:
    constexpr static const double BREAK_PROB = 0.2; // Probability for a weapon of breaking
public:
    static bool breakWeapon() {
        std::random_device rd;
        std::mt19937 gen(rd());
        std::uniform_real_distribution<double> dis(0, 1);
        return dis(gen) < BREAK_PROB;
    }
};


#endif //RIDMAZE_DICE_H
