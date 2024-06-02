//
// Created by miguevr on 5/30/24.
//

#include "Menu.h"

void Menu::moveUp() {
    if (selectedIndex > 0) {
        --selectedIndex;
        if (state == MAIN_MENU) {
            mainMenuOptions[selectedIndex].setFillColor(sf::Color::Red);
            mainMenuOptions[selectedIndex + 1].setFillColor(sf::Color::White);
        }
        else {
            settingsOptions[selectedIndex].setFillColor(sf::Color::Red);
            settingsOptions[selectedIndex + 1].setFillColor(sf::Color::White);
        }
    }
}

void Menu::moveDown() {
    if (state == MAIN_MENU && selectedIndex + 1 < MAIN_MENU_OPTIONS) {
        mainMenuOptions[selectedIndex].setFillColor(sf::Color::White);
        ++selectedIndex;
        mainMenuOptions[selectedIndex].setFillColor(sf::Color::Red);
    }
    else if (state == SETTINGS_OPTIONS && selectedIndex + 1 < MAIN_MENU_OPTIONS){
        settingsOptions[selectedIndex].setFillColor(sf::Color::White);
        ++selectedIndex;
        settingsOptions[selectedIndex].setFillColor(sf::Color::Red);
    }
}

int Menu::getSelectedIndex() const {
    return selectedIndex;
}

Menu::MenuState Menu::getState() const {
    return state;
}

void Menu::select() {
    if (state == MenuState::MAIN_MENU) {
        switch (selectedIndex) {
            case 0:
                // Implement missing parts
        }
    }
}
