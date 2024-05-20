#include "GameController.h"
#include <iostream>
#include <sstream>
#include <locale>
#include <codecvt>

GameController::GameController(int nPlayers)
        : game(nPlayers), window(sf::VideoMode(1920, 1080), "Ridmaze"), ui(std::make_unique<TextUI>()) {
    if (!font.loadFromFile("../assets/fonts/CaskaydiaCoveNerdFontMono-Regular.ttf")) {
        std::cerr << "Error loading font\n";
        exit(1);
    }
    if (!menuTexture.loadFromFile("../assets/img/ridmaze.png")) {
        std::cerr << "Error loading menu image\n";
        exit(1);
    }
    menuSprite.setTexture(menuTexture);
    menuSprite.setPosition((1920 - menuTexture.getSize().x) / 2, (1080 - menuTexture.getSize().y) / 2);

    if (!playerTexture.loadFromFile("../assets/img/player.png")) {
        std::cerr << "Error loading player image\n";
        exit(1);
    }
    playerSprite.setTexture(playerTexture);

    if (!monsterTexture.loadFromFile("../assets/img/monster.png")) {
        std::cerr << "Error loading monster image\n";
        exit(1);
    }
    monsterSprite.setTexture(monsterTexture);

    if (!blockTexture.loadFromFile("../assets/img/block.png")) {
        std::cerr << "Error loading block image\n";
        exit(1);
    }
    blockSprite.setTexture(blockTexture);

    if (!emptyTexture.loadFromFile("../assets/img/block2.png")) {
        std::cerr << "Error loading empty block image\n";
        exit(1);
    }
    emptySprite.setTexture(emptyTexture);

    if (!exitTexture.loadFromFile("../assets/img/exit.png")) {
        std::cerr << "Error loading exit block image\n";
        exit(1);
    }
    exitSprite.setTexture(exitTexture);
}

void GameController::run() {
    showMainMenu();
    gameLoop();
}

void GameController::showMainMenu() {
    window.clear();
    window.draw(menuSprite);
    window.display();

    sf::Event event;
    while (window.isOpen()) {
        while (window.pollEvent(event)) {
            if (event.type == sf::Event::Closed) {
                window.close();
            } else if (event.type == sf::Event::KeyPressed) {
                if (event.key.code == sf::Keyboard::Enter) {
                    return;
                }
            }
        }
    }
}

void GameController::gameLoop() {
    while (window.isOpen()) {
        processEvents();
        update();
        render();
    }
}

void GameController::processEvents() {
    sf::Event event;
    while (window.pollEvent(event)) {
        if (event.type == sf::Event::Closed) {
            window.close();
        } else if (event.type == sf::Event::KeyPressed) {
            handlePlayerInput(event.key.code);
        }
    }
}

void GameController::update() {
}

void GameController::render() {
    window.clear(sf::Color::Black);

    GameState state = game.getGameState();
    drawGameState(state);

    window.display();
}

void GameController::handlePlayerInput(sf::Keyboard::Key key) {
    Directions direction;
    bool validInput = true;

    switch (key) {
        case sf::Keyboard::Up:
        case sf::Keyboard::K:
            direction = Directions::UP;
            break;
        case sf::Keyboard::Down:
        case sf::Keyboard::J:
            direction = Directions::DOWN;
            break;
        case sf::Keyboard::Left:
        case sf::Keyboard::H:
            direction = Directions::LEFT;
            break;
        case sf::Keyboard::Right:
        case sf::Keyboard::L:
            direction = Directions::RIGHT;
            break;
        default:
            validInput = false;
            break;
    }

    if (validInput) {
        bool gameEnded = game.nextStep(direction);
        if (gameEnded) {
            std::cout << "The game has ended!" << std::endl;
            window.close();
        }
    }
}

void GameController::drawGameState(const GameState& state) {
    std::string labyrinth = state.getLabyrinth();
    int width = 10;
    int height = 10;
    float cellSize = 107.9f;

    blockSprite.setScale(cellSize / blockTexture.getSize().x, cellSize / blockTexture.getSize().y);
    emptySprite.setScale(cellSize / emptyTexture.getSize().x, cellSize / emptyTexture.getSize().y);
    playerSprite.setScale(cellSize / playerTexture.getSize().x, cellSize / playerTexture.getSize().y);
    monsterSprite.setScale(cellSize / monsterTexture.getSize().x, cellSize / monsterTexture.getSize().y);
    exitSprite.setScale(cellSize / exitTexture.getSize().x, cellSize / exitTexture.getSize().y);

    for (int y = 0; y < height; ++y) {
        for (int x = 0; x < width; ++x) {
            char cell = labyrinth[y * width + x];
            sf::Sprite* sprite = nullptr;

            if (cell == 'X') {
                sprite = &blockSprite;
            }
            else if (cell == '-') {
                sprite = &emptySprite;
            }
            else if (isdigit(cell)) {
                sprite = &playerSprite;
            }
            else if (cell == 'M') {
                sprite = &monsterSprite;
            }
            else if (cell == 'E') {
                sprite = &exitSprite;
            }

            if (sprite) {
                sprite->setPosition(x * cellSize, y * cellSize);
                window.draw(*sprite);
            }
        }
    }
}
