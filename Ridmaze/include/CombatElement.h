//
// Created by miguevr on 5/17/24.
//

#ifndef RIDMAZE_COMBATELEMENT_H
#define RIDMAZE_COMBATELEMENT_H


#include "Dice.h"
#include <string>
#include <string_view>
#include <sstream>

class CombatElement {
public:
    CombatElement(double effect, int uses);
    CombatElement(const CombatElement& other);

    auto produceEffect() -> double;
    [[nodiscard]] auto discard() const -> bool;

    [[nodiscard]] virtual auto toString() const -> std::string;

protected:
    double effect;
    int uses;
};



#endif //RIDMAZE_COMBATELEMENT_H
