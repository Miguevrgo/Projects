//
// Created by miguevr on 4/25/24.
//

#ifndef RIDMAZE_DICE_H
#define RIDMAZE_DICE_H

#include <random>

class Dice {
private:
    constexpr static const double BREAK_PROB = 0.2; // Probability for a weapon of breaking
    constexpr static const double MAX_STRENGTH = 5;
public:
    static bool breakWeapon();
    static double randomStrength();
    static int randomRarity();
};


#endif //RIDMAZE_DICE_H