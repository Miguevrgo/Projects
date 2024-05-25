#ifndef RIDMAZE_GAMECONTROLLER_H
#define RIDMAZE_GAMECONTROLLER_H

#include <SFML/Graphics.hpp>
#include <memory>
#include <string>
#include "Game.h"

class GameController {
public:
    GameController(const std::vector<std::string_view>& configFiles);
    void handlePlayerInput(sf::Keyboard::Key key, sf::Window& window);
    [[nodiscard]] Game getGame() const;

private:
    Game game;
};


#endif // RIDMAZE_GAMECONTROLLER_H
