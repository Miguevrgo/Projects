//
// Created by miguevr on 5/22/24.
//

#ifndef RIDMAZE_GUI_H
#define RIDMAZE_GUI_H

#include <SFML/Graphics.hpp>
#include <iostream>
#include "GameState.h"
#include "Menu.h"
#include "GameController.h"

class GUI {
public:
    GUI(int width, int height);
    void loadResources();
    void render(const GameState& state, int rows, int cols);
    void drawGameState(const GameState& state, int rows, int cols);
    void showMainMenu();
    void handleMenuInput(sf::Keyboard::Key key);
    void run();
    void startGame();
    void gameLoop();
    void processEvents();

private:
    static constexpr int rows = 18; // Not UpperCase intentionally (Multiple maps)
    static constexpr int cols = 32;

    GameController controller;

    sf::RenderWindow window;
    sf::Font font;

    sf::Texture menuTexture;
    sf::Sprite menuSprite;

    sf::Texture playerTexture;
    sf::Sprite playerSprite;
    sf::Texture monsterTexture;
    sf::Sprite monsterSprite;
    sf::Texture blockTexture;
    sf::Sprite blockSprite;
    sf::Texture emptyTexture;
    sf::Sprite emptySprite;
    sf::Texture exitTexture;
    sf::Sprite exitSprite;
    sf::Texture upStairTexture;
    sf::Sprite upStairSprite;
    sf::Texture downStairTexture;
    sf::Sprite downStairSprite;


    Menu menu;
    bool inGame;

    std::vector<sf::Texture> optionTextures;
    std::vector<sf::Sprite> optionSprites;

    int fpsCounter;
    int fps;
    sf::Text fpsText;
    sf::Clock fpsClock;
    sf::Time fpsTime;


};


#endif //RIDMAZE_GUI_H
