//
// Created by miguevr on 5/17/24.
//
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
}

void GameController::run() {
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
        case sf::Keyboard::J:
        case sf::Keyboard::Down:
            direction = Directions::DOWN;
            break;
        case sf::Keyboard::H:
        case sf::Keyboard::Left:
            direction = Directions::LEFT;
            break;
        case sf::Keyboard::L:
        case sf::Keyboard::Right:
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
    std::ostringstream oss;
    oss << state.getLabyrinth() << "\n";
    oss << "Players: \n" << state.getPlayers() << "\n";
    oss << "Monsters: \n" << state.getMonsters() << "\n";
    oss << "Log:\n" << state.getLog() << "\n";

    std::wstring_convert<std::codecvt_utf8_utf16<wchar_t>> converter;
    std::wstring wide_string = converter.from_bytes(oss.str());

    sf::Text text;
    text.setFont(font);
    text.setString(wide_string);
    text.setCharacterSize(20);
    text.setFillColor(sf::Color::Blue);

    window.draw(text);
}