/**
 * @file Render.h
 * @author Miguel Angel De la Vega Rodríguez
 * @brief Editor view class
 */

#ifndef CODE_EDITOR_RENDER_H
#define CODE_EDITOR_RENDER_H

#include <SFML/Graphics.hpp>

class Render {

    void Draw(sf::RenderWindow &window);

    void SetFontSize(int newFontSize);


    void DrawLines(sf::RenderWindow &window);
    void DrawCursor(sf::RenderWindow &window);
};


#endif //CODE_EDITOR_RENDER_H
