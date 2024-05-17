//
// Created by miguevr on 5/17/24.
//

#include "Monster.h"

Monster::Monster(std::string_view name, double intelligence, double strength) :
    LabyrinthCharacter(name, intelligence, strength, INITIAL_HEALTH) {}

auto Monster::attack() -> double {
    return Dice::intensity(getStrength());
}

auto Monster::defend(double receivedAttack) -> bool {
    bool isDead = dead();

    if (!isDead) {
        if (Dice::intensity(getIntelligence()) < receivedAttack) {
            gotWounded();
            isDead = dead();
        }
    }

    return isDead;
};


