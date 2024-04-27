//
// Created by miguevr on 4/25/24.
//

#include "Weapon.h"

int Weapon::getUses() const {
    return uses;
}

double Weapon::getDamage() const {
    return damage;
}

double Weapon::attack() {
    if (uses == 0) {
        return 0;
    }
    else{
        --uses;
        return getDamage();
    }
}

bool Weapon::discard() const {
    if (getUses() == 0) {
        return true;
    }
    else {
        return Dice::breakWeapon();
    }
}

