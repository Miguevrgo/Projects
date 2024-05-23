#include "Game.h"
#include "Dice.h"
#include <sstream>

const int Game::MONSTERS_POS[NMONSTERS][2] = {
        {1, 1}, {3, 5}, {5, 2}, {6, 7}, {8, 1},
        {10, 12}, {12, 5}, {14, 14}, {16, 3}, {17, 9}
};

const std::tuple<Orientation, int, int, int> Game::BLOCKS_POS[NBLOCKS] = {
        {Orientation::HORIZONTAL, 0, 1, 4},
        {Orientation::HORIZONTAL, 9, 0, 5},
        {Orientation::VERTICAL, 2, 8, 7},
        {Orientation::VERTICAL, 3, 3, 4},
        {Orientation::HORIZONTAL, 5, 10, 8},
        {Orientation::VERTICAL, 12, 5, 6},
        {Orientation::HORIZONTAL, 14, 14, 3},
        {Orientation::VERTICAL, 15, 0, 5},
        {Orientation::HORIZONTAL, 7, 20, 10},
        {Orientation::VERTICAL, 8, 18, 7},
        {Orientation::HORIZONTAL, 11, 6, 8},
        {Orientation::VERTICAL, 13, 4, 6},
        {Orientation::HORIZONTAL, 3, 15, 7},
        {Orientation::VERTICAL, 17, 2, 8},
        {Orientation::HORIZONTAL, 16, 8, 5},
        {Orientation::VERTICAL, 1, 12, 4},
        {Orientation::HORIZONTAL, 4, 18, 6},
        {Orientation::VERTICAL, 9, 7, 5},
        {Orientation::HORIZONTAL, 15, 10, 7},
        {Orientation::VERTICAL, 10, 14, 6}
};

Game::Game(int rows, int cols)
        : labyrinth(rows, cols, Dice::randomPos(rows), Dice::randomPos(cols)) {

    player = std::make_shared<Player>(Dice::randomIntelligence(), Dice::randomStrength());
    configureLabyrinth();
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

void Game::configureLabyrinth() {
    for (int i = 0; i < NMONSTERS; ++i) {
        auto monster = std::make_shared<Monster>("Monster " + std::to_string(i), Dice::randomIntelligence(), Dice::randomStrength());
        monsters.push_back(monster);
        labyrinth.addMonster(MONSTERS_POS[i][0], MONSTERS_POS[i][1], monster);
    }

    for (const auto& block : BLOCKS_POS) {
        labyrinth.addBlock(std::get<0>(block), std::get<1>(block), std::get<2>(block), std::get<3>(block));
    }
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
