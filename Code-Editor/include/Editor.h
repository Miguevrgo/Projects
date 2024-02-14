/**
 * @file Editor.h
 * @author Miguel Ángel De la Vega Rodríguez
 * @brief Editor class to implement useful functions
 */

#ifndef CODE_EDITOR_EDITOR_H
#define CODE_EDITOR_EDITOR_H

#include <SFML/Graphics.hpp>
#include <fstream>
#include <iostream>
#include <vector>
#include <string>
#include <sstream>

class Editor {
public:
    bool SaveFile(const std::string& fileName);
    bool HasChanged() const;

    int GetNumLines();
    int GetCharContained(int startLine, int startChar, int endLine, int endChar);

    void SwapLines(int lineA, int LineB);
private:
    std::vector<int> lineBuffer;
    sf::String buffer;
    bool documentChanged;
};


#endif //CODE_EDITOR_EDITOR_H
