//
// Created by miguevr on 5/17/24.
//

#include "CombatElement.h"

CombatElement::CombatElement(double effect, int uses)
        : effect(effect), uses(uses) {}

CombatElement::CombatElement(const CombatElement& other)
        : effect(other.effect), uses(other.uses) {}

auto CombatElement::produceEffect() -> double {
    if (uses > 0) {
        --uses;
        return effect;
    }
    return 0.0f;
}

auto CombatElement::discard() const -> bool {
    return Dice::discardElement(uses);
}

auto CombatElement::toString() const -> std::string {
    std::ostringstream oss;
    oss << "[" << effect << ", " << uses << "]";
    return oss.str();
}
