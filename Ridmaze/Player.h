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
    const static int INITIAL_HEALTH = 5;
    std::string name;
    std::vector<Weapon> weapons;
    int health;
    double strength;
public:
    Player(const std::string& name);
    double attack();
    double useWeapon();
    void discardWeapon(const Weapon &weapon);
    void addWeapon(const Weapon& weapon);
    std::string toString() const;
};


#endif //RIDMAZE_PLAYER_H
