//
// Created by miguevr on 5/17/24.
//

#include "Shield.h"

Shield::Shield(double protection, int uses)
        : CombatElement(protection, uses) {}

auto Shield::protect() -> double {
    return produceEffect();
}

auto Shield::toString() const -> std::string {
    std::ostringstream oss;
    oss << "S" << CombatElement::toString();
    return oss.str();
}