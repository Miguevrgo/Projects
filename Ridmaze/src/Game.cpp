#include "Game.h"
#include "Dice.h"
#include <sstream>


Game::Game(const std::vector<std::string_view>& configFiles)
        : labyrinth(configFiles) {

    player = std::make_shared<Player>(Dice::randomIntelligence(), Dice::randomStrength());
    labyrinth.placePlayer(player);

    std::ostringstream oss;
    oss << "╔════════════════════════════╗\n"
        << "║    The Game has started!   ║\n"
        << "╚════════════════════════════╝\n\n";
    log = oss.str();
}

bool Game::finished() const {
    return labyrinth.haveAWinner();
}

bool Game::nextStep(Directions preferredDirection) {
    log.clear();
    if (player->dead()) {
        manageResurrection();
    } else {
        Directions direction = actualDirection(preferredDirection);
        if (direction != preferredDirection) {
            logPlayerNoOrders();
        }
        auto monster = labyrinth.movePlayer(direction);
        if (monster == nullptr) {
            logNoMonster();
        } else {
            auto winner = combat(monster);
            manageReward(winner);
        }
    }

    return finished();
}

GameState Game::getGameState() const {
    std::string playerInfo = player->toString() + "\n";
    std::string monstersInfo;

    for (const auto& m : monsters) {
        monstersInfo += m->toString() + "\n";
    }

    return {labyrinth.toString(), playerInfo, monstersInfo, finished(), log};
}

Directions Game::actualDirection(Directions preferredDirection) {
    int currentRow = player->getRow();
    int currentCol = player->getCol();

    auto validMoves = labyrinth.validMoves(currentRow, currentCol);
    return player->move(preferredDirection, validMoves);
}

GameCharacter Game::combat(const std::shared_ptr<Monster>& monster) {
    int rounds = 0;
    GameCharacter winner = GameCharacter::PLAYER;
    bool lose = monster->defend(player->attack());

    while (!lose && rounds < MAX_ROUNDS) {
        winner = GameCharacter::MONSTER;
        rounds++;
        lose = player->defend(monster->attack());
        if (!lose) {
            winner = GameCharacter::PLAYER;
            lose = monster->defend(player->attack());
        }
    }
    logRounds(rounds, MAX_ROUNDS);
    return winner;
}

void Game::manageReward(GameCharacter winner) {
    if (winner == GameCharacter::PLAYER) {
        player->receiveReward();
        logPlayerWon();
    } else {
        logMonsterWon();
    }
}

void Game::manageResurrection() {
    if (Dice::resurrectPlayer()) {
        player->resurrect();
        logResurrected();
    } else {
        logPlayerSkipTurn();
    }
}

void Game::logPlayerWon() {
    log += "The player has won\n";
}

void Game::logMonsterWon() {
    log += "The monster has won\n";
}

void Game::logResurrected() {
    log += "The player has been resurrected\n";
}

void Game::logPlayerSkipTurn() {
    log += "The player has skipped their turn for being dead\n";
}

void Game::logPlayerNoOrders() {
    log += "The player hasn't followed the orders\n";
}

void Game::logNoMonster() {
    log += "Player has moved to an empty cell or has not moved\n";
}

void Game::logRounds(int rounds, int max) {
    std::ostringstream oss;
    oss << "Rounds: " << rounds << "|" << max << "\n";
    log += oss.str();
}
