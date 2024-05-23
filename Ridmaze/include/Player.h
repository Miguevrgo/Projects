//
// Created by miguevr on 5/17/24.
//

#ifndef RIDMAZE_PLAYER_H
#define RIDMAZE_PLAYER_H

#include <vector>
#include <algorithm>
#include <sstream>
#include "Dice.h"
#include "Weapon.h"
#include "Shield.h"
#include "LabyrinthCharacter.h"
#include "Directions.h"

class Player : public LabyrinthCharacter {
public:
    Player(double intelligence, double strength);
    Player(const Player& rhs);

    void resurrect();
    static auto move(Directions direction, const std::vector<Directions>& validMoves) -> Directions;

    auto attack() -> double override;
    auto defend(double receivedAttack) -> bool override;

    void receiveReward();
    [[nodiscard]] auto toString() const -> std::string override;

private:
    int consecutiveHits;
    std::vector<Weapon> weapons;
    std::vector<Shield> shields;
    static constexpr int INITIAL_HEALTH = 10;

    void receiveWeapon(const Weapon& weapon);
    void receiveShield(const Shield& shield);
    static auto newWeapon() -> Weapon;
    static auto newShield() -> Shield;
    [[nodiscard]] auto sumWeapons() const -> double;
    [[nodiscard]] auto sumShields() const -> double;
    [[nodiscard]] auto defensiveEnergy() const -> double;
    auto manageHit(double receivedAttack) -> bool;
    void resetHits();
    void incConsecutiveHits();
};


#endif //RIDMAZE_PLAYER_H
