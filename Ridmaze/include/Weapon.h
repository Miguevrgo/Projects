//
// Created by miguevr on 5/17/24.
//

#ifndef RIDMAZE_WEAPON_H
#define RIDMAZE_WEAPON_H

#include "CombatElement.h"

class Weapon : public CombatElement {
public:
    Weapon(double power, int uses);
    auto attack() -> double;
    [[nodiscard]] auto toString() const -> std::string override;
};

#endif //RIDMAZE_WEAPON_H
