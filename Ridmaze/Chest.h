//
// Created by miguevr on 4/28/24.
//

#ifndef RIDMAZE_CHEST_H
#define RIDMAZE_CHEST_H

#include "Player.h"

class Chest {
private:
    enum class Rarity {
        COMMON,
        RARE,
        EPIC,
        LEGENDARY
    };
    Rarity rarity;
    bool opened = false;
public:
    Chest(Rarity rarity) : rarity(rarity){};
    Rarity getRarity() const;
    void getLoot(Player &player);
    bool isOpened() const;
};


#endif //RIDMAZE_CHEST_H
