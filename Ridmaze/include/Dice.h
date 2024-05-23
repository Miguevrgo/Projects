//
// Created by miguevr on 5/16/24.
//

#ifndef RIDMAZE_DICE_H
#define RIDMAZE_DICE_H

#include <random>
#include <vector>
#include "Directions.h"

class Dice {
public:
    [[nodiscard]] static auto randomPos(int max) -> int;
    [[nodiscard]] static auto randomIntelligence() -> double;
    [[nodiscard]] static auto randomStrength() -> double;
    [[nodiscard]] static auto resurrectPlayer() -> bool;
    [[nodiscard]] static auto shieldsReward() -> int;
    [[nodiscard]] static auto weaponsReward() -> int;
    [[nodiscard]] static auto healthReward() -> int;
    [[nodiscard]] static auto weaponPower() -> double;
    [[nodiscard]] static auto shieldPower() -> double;
    [[nodiscard]] static auto usesLeft() -> int;
    [[nodiscard]] static auto intensity(double competence) -> double;
    [[nodiscard]] static auto discardElement(int usesLeft) -> bool;
    [[nodiscard]] static auto nextStep(Directions preference, const std::vector<Directions>& validMoves, double intelligence) -> Directions;

private:

    static std::mt19937 randomGenerator;
    static constexpr int MAX_USES = 5; // Maximum uses for both the weapons and shields
    static constexpr double MAX_INTELLIGENCE = 10.0f; // Maximum intelligence for the player and the enemy
    static constexpr double MAX_STRENGTH = 10.0f; // Maximum strength for the player and the enemy
    static constexpr double RESURRECT_PROB = 0.3f; // Probability of resurrecting after each round
    static constexpr int WEAPONS_REWARD = 2; // Maximum number of weapons rewarded when winning a combat
    static constexpr int SHIELDS_REWARD = 3; // Maximum number of shields rewarded when winning a combat
    static constexpr int HEALTH_REWARD = 5; // Maximum health rewarded when winning a combat
    static constexpr int MAX_ATTACK = 3; // Maximum attack power for the weapons
    static constexpr int MAX_SHIELD = 2; // Maximum shield power for the shields
};


#endif //RIDMAZE_DICE_H
