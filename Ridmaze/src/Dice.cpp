//
// Created by miguevr on 5/16/24.
//

#include "Dice.h"

std::mt19937 Dice::randomGenerator{std::random_device{}()};

auto Dice::randomPos(int max) -> int {
    return std::uniform_int_distribution<int>{0, max}(randomGenerator);
}

auto Dice::whoStarts(int nPlayers) -> int {
    return std::uniform_int_distribution<int>{0, nPlayers - 1}(randomGenerator);
}

auto Dice::randomIntelligence() -> double {
    return std::uniform_real_distribution<double>{0.0, MAX_INTELLIGENCE}(randomGenerator);
}

auto Dice::randomStrength() -> double {
    return std::uniform_real_distribution<double>{0.0, MAX_STRENGTH}(randomGenerator);
}

auto Dice::resurrectPlayer() -> bool {
    return std::bernoulli_distribution{RESURRECT_PROB}(randomGenerator);
}

auto Dice::shieldsReward() -> int {
    return std::uniform_int_distribution<int>{0, SHIELDS_REWARD}(randomGenerator);
}

auto Dice::weaponsReward() -> int {
    return std::uniform_int_distribution<int>{0, WEAPONS_REWARD}(randomGenerator);
}

auto Dice::healthReward() -> int {
    return std::uniform_int_distribution<int>{0, HEALTH_REWARD}(randomGenerator);
}

auto Dice::weaponPower() -> double {
    return std::uniform_real_distribution<double>{0.0, MAX_ATTACK}(randomGenerator);
}

auto Dice::shieldPower() -> double {
    return std::uniform_real_distribution<double>{0.0, MAX_SHIELD}(randomGenerator);
}

auto Dice::usesLeft() -> int {
    return std::uniform_int_distribution<int>{0, MAX_USES}(randomGenerator);
}

auto Dice::intensity(double competence) -> double {
    return std::uniform_real_distribution<double>{0.0, competence}(randomGenerator);
}

auto Dice::discardElement(int usesLeft) -> bool {
    std::uniform_real_distribution<double> distribution(0.0, 1.0);
    return distribution(randomGenerator) >= static_cast<float>(usesLeft) / MAX_USES;
}

auto Dice::nextStep(Directions preference, const std::vector<Directions> &validMoves, double intelligence) -> Directions {
    if (intelligence >= randomIntelligence()) {
        return preference;
    }

    std::uniform_int_distribution<int> distribution(0, static_cast<int>(validMoves.size()) - 1);
    return validMoves[distribution(randomGenerator)];
}


























