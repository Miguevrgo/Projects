//
// Created by miguevr on 5/30/24.
//

#include "Settings.h"


Settings::Settings(bool showFPS, int width, int height) : showFPS(showFPS), screenWidth(width), screenHeight(height) {}

bool Settings::isFPSEnabled() const {
    return showFPS;
}

void Settings::toggleFPS() {
    showFPS = !showFPS;
}

std::pair<int, int> Settings::getResolution() const {
    return {screenWidth, screenHeight};
}

void Settings::setResolution(int width, int height) {
    screenWidth = width;
    screenHeight = height;
}
