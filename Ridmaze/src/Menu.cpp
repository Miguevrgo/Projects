//
// Created by miguevr on 5/28/24.
//

#include "Menu.h"

Menu::Menu(int width, int height) : selectedIndex(0) {}

void Menu::draw(sf::RenderWindow &window) {
    for (const auto& item : menu) {
        window.draw(item);
    }
}


void Menu::moveUp() {
    if (selectedIndex - 1 >= 0) {
        selectedIndex--;
    }
}

void Menu::moveDown() {
    if (selectedIndex + 1 < NUM_OPTIONS) {
        selectedIndex++;
    }
}

void Menu::setMenuSprites(const std::vector<sf::Sprite>& sprites) {
    menu = sprites;
}

int Menu::getSelectedIndex() const noexcept {
    return selectedIndex;
}
