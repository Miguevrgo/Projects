//
// Created by miguevr on 5/17/24.
//

#ifndef RIDMAZE_SHIELD_H
#define RIDMAZE_SHIELD_H

#include "CombatElement.h"

class Shield : public CombatElement {
public:
    Shield(double protection, int uses);
    auto protect() -> double;
    [[nodiscard]] auto toString() const -> std::string override;
};


#endif //RIDMAZE_SHIELD_H
