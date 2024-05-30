//
// Created by miguevr on 5/30/24.
//

#ifndef RIDMAZE_MENU_H
#define RIDMAZE_MENU_H

#include <SFML/Graphics.hpp>
#include <vector>
#include <string>
#include "Settings.h"

class Menu {
public:
    Menu(float width, float height, Settings& settings);
    void draw(sf::RenderWindow &window);
    void moveUp();
    void moveDown();
    void select();
    int getSelectedIndex() const;

    enum MenuState { MAIN_MENU, SETTINGS, EXIT };
    MenuState getState() const;

private:
    void loadTextures();
    void loadFonts();

    std::vector<sf::Text> mainMenuOptions;
    std::vector<sf::Text> settingsOptions;
    sf::Font font;

    int selectedIndex;
    MenuState state;

    Settings& settings;

    static constexpr int MAIN_MENU_OPTIONS = 3;
    static constexpr int SETTINGS_OPTIONS = 3;
};


#endif //RIDMAZE_MENU_H
