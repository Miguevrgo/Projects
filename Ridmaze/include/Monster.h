//
// Created by miguevr on 5/17/24.
//

#ifndef RIDMAZE_MONSTER_H
#define RIDMAZE_MONSTER_H

#include "LabyrinthCharacter.h"
#include "Dice.h"

class Monster : public LabyrinthCharacter {
public:
    Monster(std::string_view name, double intelligence, double strength);

    auto attack() -> double override;
    auto defend(double receivedAttack) -> bool override;

private:
    static constexpr int INITIAL_HEALTH = 10;
};


#endif //RIDMAZE_MONSTER_H
