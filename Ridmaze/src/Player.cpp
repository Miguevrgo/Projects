/**
 * @file Player.cpp
 * @author Miguel Angel De la Vega Rodríguez
 * @brief Class implementation for Player class
 */

#include "Player.h"

Player::Player(const std::string& name) {
    this->name = name;
    this->health = INITIAL_HEALTH;
    this->strength = Dice::randomStrength();
}

void Player::addWeapon(const Weapon &weapon) {
    if (std::find(weapons.begin(), weapons.end(), weapon) == weapons.end()) {
        weapons.emplace_back(weapon);
    }
}

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

std::string Player::toString() const {
    std::string toReturn = name + ": ";
    for (auto &weapon : weapons) {
        toReturn += "\t" + weapon.toString();
    }
    toReturn += std::to_string(health) + "♥" + std::to_string(strength) + "⚔";
    return toReturn;
}

void Player::setHealth(int health) {
    this->health = health;
}

int Player::getHealth() const {
    return health;
}

void Player::incStrength(double amount) {
    this->strength += amount;
}


