/**
 * @file Player.h
 * @brief Class definition for Player
 */

#ifndef RIDMAZE_PLAYER_H
#define RIDMAZE_PLAYER_H

#include <vector>
#include <algorithm>
#include "Weapon.h"

class Player {
private:
    int health;
    std::vector<Weapon> weapons;
    double strength;
public:
    double attack();
    double useWeapon();
    void discardWeapon(const Weapon &weapon);
};


#endif //RIDMAZE_PLAYER_H
