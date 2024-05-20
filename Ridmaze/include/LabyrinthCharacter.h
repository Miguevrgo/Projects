//
// Created by miguevr on 5/16/24.
//

#ifndef RIDMAZE_LABYRINTHCHARACTER_H
#define RIDMAZE_LABYRINTHCHARACTER_H

#include <string>
#include <string_view>

class LabyrinthCharacter {
public:
    LabyrinthCharacter(std::string_view name, double intelligence, double strength, double health);
    LabyrinthCharacter(const LabyrinthCharacter& rhs);

    [[nodiscard]] auto dead() const -> bool;
    [[nodiscard]] auto getRow() const -> int;
    [[nodiscard]] auto getCol() const -> int;
    [[nodiscard]] auto getIntelligence() const -> double;
    [[nodiscard]] auto getStrength() const -> double;
    [[nodiscard]] auto getHealth() const -> double;

    void setHealth(double health);
    void setPos(int row, int col);
    virtual auto attack() -> double = 0;
    virtual auto defend(double attack) -> bool = 0;

    [[nodiscard]] virtual auto toString() const -> std::string;

protected:
    void gotWounded();

private:
    std::string name;
    double intelligence;
    double strength;
    double health;
    int row;
    int col;
};


#endif //RIDMAZE_LABYRINTHCHARACTER_H
