//
// Created by miguevr on 5/28/24.
//

#ifndef RIDMAZE_MENU_H
#define RIDMAZE_MENU_H

#include <SFML/Graphics.hpp>
#include <array>
#include <string>
#include <iostream>

class Menu {
public:
    Menu(int width, int height);
    void draw(sf::RenderWindow& window);
    void moveUp();
    void setMenuSprites(const std::vector<sf::Sprite>& sprites);
    void moveDown();
    [[nodiscard]] int getSelectedIndex() const noexcept;
private:
    static constexpr int NUM_OPTIONS = 3;
    std::vector<sf::Sprite> menu;
    int selectedIndex;
};


#endif //RIDMAZE_MENU_H
