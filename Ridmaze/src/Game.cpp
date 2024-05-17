//
// Created by miguevr on 5/17/24.
//

#include "Game.h"
#include "Dice.h"
#include <sstream>

const int Game::MONSTERS_POS[NMONSTERS][2] = {{0,0},{1,6},{2,2},{5,5},{7,1}};
const std::tuple<Orientation, int, int, int> Game::BLOCKS_POS[NBLOCKS] = {
        {Orientation::HORIZONTAL, 0, 1, 4},
        {Orientation::HORIZONTAL, 9, 0, 2},
        {Orientation::VERTICAL, 2, 8, 7},
        {Orientation::VERTICAL, 3, 3, 4}
};

Game::Game(int nPlayers)
        : labyrinth(ROWS, COLS, Dice::randomPos(ROWS), Dice::randomPos(COLS)),
          currentPlayerIndex(Dice::whoStarts(nPlayers)),
          currentPlayer(nullptr) {

    for (int playerNum = 0; playerNum < nPlayers; ++playerNum) {
        players.emplace_back(std::make_shared<Player>(playerNum, Dice::randomIntelligence(), Dice::randomStrength()));
    }

    currentPlayer = players[currentPlayerIndex];
    configureLabyrinth();
    labyrinth.spreadPlayers(players);

    std::ostringstream oss;
    oss << "╔════════════════════════════╗\n"
        << "║    The Game has started!   ║\n"
        << "╚════════════════════════════╝\n\n";
    log = oss.str();
}

auto Game::finished() const -> bool {
    return labyrinth.haveAWinner();
}

auto Game::nextStep(Directions preferredDirection) -> bool {
    log = "";
    if (currentPlayer->dead()) {
        manageResurrection();
    } else {
        auto direction = actualDirection(preferredDirection);
        if (direction != preferredDirection) {
            logPlayerNoOrders();
        }
        auto monster = labyrinth.putPlayer(direction, currentPlayer);
        if (monster == nullptr) {
            logNoMonster();
        } else {
            auto winner = combat(monster);
            manageReward(winner);
        }
    }

    auto endGame = finished();
    if (!endGame) {
        nextPlayer();
    }
    return endGame;
}

auto Game::getGameState() const -> GameState {
    std::string player;
    std::string monster;

    for (const auto& p : players) {
        player += p->toString() + "\n";
    }
    for (const auto& m : monsters) {
        monster += m->toString() + "\n";
    }

    return {labyrinth.toString(), player, monster, currentPlayerIndex, finished(), log};
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

void Game::nextPlayer() {
    currentPlayerIndex = (currentPlayerIndex + 1) % players.size();
    currentPlayer = players[currentPlayerIndex];
}

auto Game::actualDirection(Directions preferredDirection) -> Directions {
    int currentRow = currentPlayer->getRow();
    int currentCol = currentPlayer->getCol();

    auto validMoves = labyrinth.validMoves(currentRow, currentCol);
    return currentPlayer->move(preferredDirection, validMoves);
}

auto Game::combat(std::shared_ptr<Monster> monster) -> GameCharacter {
    int rounds = 0;
    GameCharacter winner = GameCharacter::PLAYER;
    bool lose = monster->defend(currentPlayer->attack());

    while (!lose && rounds < MAX_ROUNDS) {
        winner = GameCharacter::MONSTER;
        rounds++;
        lose = currentPlayer->defend(monster->attack());
        if (!lose) {
            winner = GameCharacter::PLAYER;
            lose = monster->defend(currentPlayer->attack());
        }
    }
    logRounds(rounds, MAX_ROUNDS);
    return winner;
}

void Game::manageReward(GameCharacter winner) {
    if (winner == GameCharacter::PLAYER) {
        currentPlayer->receiveReward();
        logPlayerWon();
    } else {
        logMonsterWon();
    }
}

void Game::manageResurrection() {
    if (Dice::resurrectPlayer()) {
        currentPlayer->resurrect();
        logResurrected();
        auto fuzzyPlayer = std::make_shared<Player>(*currentPlayer);
        players[currentPlayerIndex] = fuzzyPlayer;
        labyrinth.updatePos(fuzzyPlayer);
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
    log += "The player has skipped his turn for being dead\n";
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
