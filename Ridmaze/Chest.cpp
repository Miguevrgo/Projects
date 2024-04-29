//
// Created by miguevr on 4/28/24.
//

#include "Chest.h"


bool Chest::isOpened() const {
    return opened;
}

Chest::Rarity Chest::getRarity() const {
    return this->rarity;
}

void Chest::getLoot(Player &player) {
    if (!isOpened()) {
        opened = true;
        switch (rarity) {
            case Rarity::COMMON:
                player.setHealth(player.getHealth() + 10);
                break;
            case Rarity::RARE:
                player.incStrength(Dice::randomStrength()/2.0);
                break;
            case Rarity::EPIC:
                player.addWeapon(Weapon()) // Add a weapon from a list of predefined ones with rarity
                // This also involves changing rarity visibility for Dice and for Chest to a separate enum file
                break;
            case Rarity::LEGENDARY:
                player.addWeapon(Weapon(Dice::randomRarity()));
                break;
        }
    }
}
