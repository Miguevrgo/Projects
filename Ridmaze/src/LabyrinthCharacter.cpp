//
// Created by miguevr on 5/16/24.
//

#include <sstream>
#include "LabyrinthCharacter.h"

LabyrinthCharacter::LabyrinthCharacter(std::string_view name, double intelligence, double strength, double health) :
    name(name), intelligence(intelligence), strength(strength), health(health), row(0), col(0) {}

LabyrinthCharacter::LabyrinthCharacter(const LabyrinthCharacter &rhs) :
        name(rhs.name), intelligence(rhs.intelligence), strength(rhs.strength), health(rhs.health), row(rhs.row), col(rhs.col) {}

auto LabyrinthCharacter::dead() const -> bool {
    return health <= 0;
}

auto LabyrinthCharacter::getRow() const -> int {
    return row;
}

auto LabyrinthCharacter::getCol() const -> int {
    return col;
}

auto LabyrinthCharacter::getIntelligence() const -> double {
    return intelligence;
}

auto LabyrinthCharacter::getStrength() const -> double {
    return strength;
}

auto LabyrinthCharacter::getHealth() const -> double {
    return health;
}

void LabyrinthCharacter::setHealth(double health) {
    this->health = health;
}

void LabyrinthCharacter::setPos(int row, int col) {
    this->row = row;
    this->col = col;
}

auto LabyrinthCharacter::toString() const -> std::string {
    std::ostringstream oss;
    oss << "[" << name << ", " << health << "♥, " << intelligence << ", " << strength << "]\n";
    return oss.str();
}

void LabyrinthCharacter::gotWounded() {
    --health;
}


