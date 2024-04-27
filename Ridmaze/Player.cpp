//
// Created by miguevr on 4/27/24.
//

#include "Player.h"

double Player::attack() {
    double totalDamage = 0;

    for (auto &weapon : weapons) {
        totalDamage += weapon.attack();
        discardWeapon(weapon);
    }
}

void Player::discardWeapon(const Weapon &weapon) {
    if (weapon.discard()) {
        weapons.erase(std::find(weapons.begin(), weapons.end(), weapon));
    }
}
