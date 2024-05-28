//
// Created by miguevr on 5/22/24.
//

#include "GUI.h"

GUI::GUI(int width, int height) : window(sf::VideoMode(width, height), "Ridmaze"), inGame(false),
    controller({"../assets/levels/level0.txt","../assets/levels/level1.txt"}), fps(0), fpsCounter(0), menu(width, height)
{
        loadResources();
}

void GUI::loadResources() {
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

    std::vector<std::string> fileNames = {
            "../assets/img/start.png",
            "../assets/img/settings.png",
            "../assets/img/exit_game.png",
    };

    int pos = 100;
    for (const auto& fileName : fileNames) {
        sf::Texture texture;
        if (!texture.loadFromFile(fileName)) {
            std::cerr << "Error loading" << fileName << '\n';
            exit(1);
        }

        optionTextures.push_back(texture);

        sf::Sprite sprite;
        sprite.setTexture(optionTextures.back());
        sprite.setPosition((1920 - texture.getSize().x) / 2, ((1080 - texture.getSize().y) / 2) + pos);
        pos += texture.getSize().y + 40;
        optionSprites.push_back(sprite);
    }

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

    if (!upStairTexture.loadFromFile("../assets/img/stair1.png")) {
        std::cerr << "Error loading stair image\n";
        exit(1);
    }
    upStairSprite.setTexture(upStairTexture);

    if (!downStairTexture.loadFromFile("../assets/img/stair.png")) {
        std::cerr << "Error loading stair down image\n";
        exit(1);
    }
    downStairSprite.setTexture(downStairTexture);

    fpsText.setFont(font);
    fpsText.setCharacterSize(20);
    fpsText.setFillColor(sf::Color::White);
    fpsText.setPosition(10, 10);

    std::vector<sf::Sprite> menuSprites;
    for (int i = 0; i < 3; ++i) {
        menuSprites[i] = optionSprites[i];
    }
    menu.setMenuSprites(menuSprites);
}

void GUI::render(const GameState &state, int rows, int cols) {
    window.clear(sf::Color::Black);

    drawGameState(state, rows, cols);

    fpsCounter++;
    fpsTime = fpsClock.getElapsedTime();
    if (fpsTime.asSeconds() >= 1.0f) {
        fps = fpsCounter;
        fpsCounter = 0;
        fpsClock.restart();
    }

    fpsText.setString("FPS: " + std::to_string(fps));
    window.draw(fpsText);

    window.display();
}

void GUI::drawGameState(const GameState &state, int rows, int cols) {
    std::string labyrinth = state.getLabyrinth();
    float cellSize = 60.0f;

    blockSprite.setScale(cellSize / blockTexture.getSize().x, cellSize / blockTexture.getSize().y);
    emptySprite.setScale(cellSize / emptyTexture.getSize().x, cellSize / emptyTexture.getSize().y);
    playerSprite.setScale(cellSize / playerTexture.getSize().x, cellSize / playerTexture.getSize().y);
    monsterSprite.setScale(cellSize / monsterTexture.getSize().x, cellSize / monsterTexture.getSize().y);
    exitSprite.setScale(cellSize / exitTexture.getSize().x, cellSize / exitTexture.getSize().y);
    upStairSprite.setScale(cellSize/ upStairTexture.getSize().x, cellSize / upStairTexture.getSize().y);
    downStairSprite.setScale(cellSize/ downStairTexture.getSize().x, cellSize / downStairTexture.getSize().y);

    for (int y = 0; y < rows; ++y) {
        for (int x = 0; x < cols; ++x) {
            char cell = labyrinth[y * cols + x];
            sf::Sprite* sprite = nullptr;

            if (cell == 'X') {
                sprite = &blockSprite;
            }
            else if (cell == '-' || cell == 'P' || cell == 'M') {
                sprite = &emptySprite;
            }
            else if (cell == 'E') {
                sprite = &exitSprite;
            }
            else if(cell == 'U') {
                sprite = &upStairSprite;
            }
            else if(cell == 'D') {
                sprite = &downStairSprite;
            }

            if (sprite) {
                sprite->setPosition(x * cellSize, y * cellSize);
                window.draw(*sprite);
            }
        }
    }

    for (int y = 0; y < rows; ++y) {
        for (int x = 0; x < cols; ++x) {
            char cell = labyrinth[y * cols + x];
            sf::Sprite* sprite = nullptr;
            if (cell == 'P') {
                sprite = &playerSprite;
            } else if (cell == 'M') {
                sprite = &monsterSprite;
            }

            if (sprite) {
                sprite->setPosition(x * cellSize, y * cellSize);
                window.draw(*sprite);
            }
        }
    }
}

void GUI::showMainMenu() {
    while (window.isOpen() && !inGame) {
        window.clear();
        window.draw(menuSprite);
        for (const auto& sprite : optionSprites) {
            window.draw(sprite);
        }
        menu.draw(window);
        window.display();

        sf::Event event{};
        while (window.pollEvent(event)) {
            if (event.type == sf::Event::Closed) {
                window.close();
            } else if (event.type == sf::Event::KeyPressed) {
                handleMenuInput(event.key.code);
            }
        }
    }
}

void GUI::handleMenuInput(sf::Keyboard::Key key) {
    if (key == sf::Keyboard::Up) {
        menu.moveUp();
    }
    else if (key == sf::Keyboard::Down) {
        menu.moveDown();
    }
    else if(key == sf::Keyboard::Enter) {
        int selectedItem = menu.getSelectedIndex();

        switch (selectedItem) {
            case 0:
                startGame();
                break;
            case 1:
                startGame();
                break;
            default:
                window.close();
                break;
        }
    }

    window.clear();
    window.draw(menuSprite);
    for (const auto& sprite : optionSprites) {
        window.draw(sprite);
    }
    menu.draw(window);
    window.display();
}

void GUI::startGame() {
    inGame = true;
    gameLoop();
}

void GUI::run() {
    showMainMenu();
}

void GUI::processEvents() {
    sf::Event event;
    while (window.pollEvent(event)) {
        if (event.type == sf::Event::Closed) {
            window.close();
        } else if (event.type == sf::Event::KeyPressed) {
            controller.handlePlayerInput(event.key.code, window);
        }
    }
}

void GUI::gameLoop() {
    while (window.isOpen()) {
        processEvents();
        render(controller.getGame().getGameState(), rows, cols);
    }
}

