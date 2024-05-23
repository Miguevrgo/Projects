//
// Created by miguevr on 5/17/24.
//

#include "Player.h"

Player::Player(double intelligence, double strength) :
    LabyrinthCharacter("Player ", intelligence, strength, INITIAL_HEALTH), consecutiveHits(0) {};

Player::Player(const Player &rhs) :
    LabyrinthCharacter(rhs), consecutiveHits(0) {}

void Player::resurrect() {
    setHealth(10);
    weapons.clear();
    shields.clear();
    resetHits();
}

auto Player::move(Directions direction, const std::vector<Directions> &validMoves) -> Directions {
    if (!validMoves.empty() && std::find(validMoves.begin(), validMoves.end(), direction) == validMoves.end()) {
        return validMoves[0];
    }

    return direction;
}

auto Player::attack() -> double {
    return sumWeapons() + this->getStrength();
}

auto Player::defend(double receivedAttack) -> bool {
    return manageHit(receivedAttack);
}

void Player::receiveReward() {
    int wReward = Dice::weaponsReward();
    int sReward = Dice::shieldsReward();

    for (int i = 0; i < wReward; ++i) {
        receiveWeapon(newWeapon());
    }
    for (int i = 0; i < sReward; ++i) {
        receiveShield(newShield());
    }
    setHealth(getHealth() + Dice::healthReward());
}

auto Player::toString() const -> std::string {
    std::ostringstream oss;
    oss << "P" << LabyrinthCharacter::toString();
    oss << "\tWeapons: ";
    for (const auto& weapon : weapons) {
        oss << weapon.toString() << "\t";
    }
    oss << "\n\tShields: ";
    for (const auto& shield : shields) {
        oss << shield.toString() << "\t";
    }
    return oss.str();
}

void Player::receiveWeapon(const Weapon& weapon) {
    weapons.erase(std::remove_if(weapons.begin(), weapons.end(), [](const Weapon& w) { return w.discard(); }), weapons.end());
    if (weapons.size() < 2) {
        weapons.push_back(weapon);
    }
}

void Player::receiveShield(const Shield& shield) {
    shields.erase(std::remove_if(shields.begin(), shields.end(), [](const Shield& s) { return s.discard(); }), shields.end());
    if (shields.size() < 3) {
        shields.push_back(shield);
    }
}

auto Player::newWeapon() -> Weapon {
    return {Dice::weaponPower(), Dice::usesLeft()};
}

auto Player::newShield() -> Shield {
    return {Dice::shieldPower(), Dice::usesLeft()};
}

auto Player::sumWeapons() const -> double {
    return std::accumulate(weapons.begin(), weapons.end(), 0.0f, [](double sum, Weapon w) { return sum + w.attack(); });
}

auto Player::sumShields() const -> double {
    return std::accumulate(shields.begin(), shields.end(), 0.0f, [](double sum, Shield s) { return sum + s.protect(); });
}

auto Player::defensiveEnergy() const -> double {
    return sumShields() + getIntelligence();
}

auto Player::manageHit(double receivedAttack) -> bool {
    if (defensiveEnergy() < receivedAttack) {
        gotWounded();
        incConsecutiveHits();
    } else {
        resetHits();
    }

    bool lose = (consecutiveHits == 3) || dead();
    if (lose) {
        resetHits();
    }
    return lose;
}

void Player::resetHits() {
    consecutiveHits = 0;
}

void Player::incConsecutiveHits() {
    ++consecutiveHits;
}


