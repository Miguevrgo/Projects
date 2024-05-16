/**
 * @file Player.h
 * @brief Class definition for Player
 */

#ifndef RIDMAZE_PLAYER_H
#define RIDMAZE_PLAYER_H

#include <vector>
#include <algorithm>
#include <string>
#include "Weapon.h"

class Player {
private:
    const static int INITIAL_HEALTH = 100;
    std::string name;
    std::vector<Weapon> weapons;
    int health;
    double strength;
public:
    explicit Player(const std::string& name);
    double attack();
    [[nodiscard]] int getHealth() const;
    void setHealth(int health);
    void incStrength(double amount);
    void discardWeapon(const Weapon &weapon);
    void addWeapon(const Weapon& weapon);
    [[nodiscard]] std::string toString() const;
};


#endif //RIDMAZE_PLAYER_H
