//
// Created by miguevr on 5/30/24.
//

#ifndef RIDMAZE_SETTINGS_H
#define RIDMAZE_SETTINGS_H

#include <utility>

class Settings {
public:
    Settings(bool showFPS, int width, int height);

    [[nodiscard]] bool isFPSEnabled() const;
    void toggleFPS();

    [[nodiscard]] std::pair<int, int> getResolution() const;
    void setResolution(int width, int height);

private:
    bool showFPS;
    int screenWidth;
    int screenHeight;
};


#endif //RIDMAZE_SETTINGS_H
