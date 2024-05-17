//
// Created by miguevr on 5/17/24.
//

#include "Weapon.h"

Weapon::Weapon(double power, int uses)
        : CombatElement(power, uses) {}

auto Weapon::attack() -> double {
    return produceEffect();
}

auto Weapon::toString() const -> std::string {
    std::ostringstream oss;
    oss << "W" << CombatElement::toString();
    return oss.str();
}