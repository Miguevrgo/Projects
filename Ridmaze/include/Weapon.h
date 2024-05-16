/**
 * @file Weapon.h
 * @author Miguel Angel De la Vega Rodríguez
 */

#ifndef RIDMAZE_WEAPON_H
#define RIDMAZE_WEAPON_H

#include <string>
#include "Dice.h"

class Weapon {
private:
    int uses;
    double damage;
    std::string name;
public:
    Weapon(int uses, double damage) : uses(uses), damage(damage){};
    int getUses() const;
    double getDamage() const;
    double attack();
    bool discard() const;
    std::string toString() const;
    bool operator==(const Weapon &rhs) const {
        return name == rhs.name && uses == rhs.uses && damage == rhs.damage;
    }
};


#endif //RIDMAZE_WEAPON_H
