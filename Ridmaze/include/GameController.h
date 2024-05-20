#ifndef RIDMAZE_GAMECONTROLLER_H
#define RIDMAZE_GAMECONTROLLER_H

#include <SFML/Graphics.hpp>
#include <memory>
#include <string>
#include "TextUI.h"
#include "Game.h"

class GameController {
public:
    GameController(int nPlayers);
    void run();

private:
    void processEvents();
    void update();
    void render();
    void handlePlayerInput(sf::Keyboard::Key key);
    void drawGameState(const GameState& state);
    void showMainMenu();
    void gameLoop();

    Game game;
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

    std::unique_ptr<TextUI> ui;
};


#endif // RIDMAZE_GAMECONTROLLER_H
