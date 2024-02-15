/**
 * @file Editor.cpp
 * @author Miguel Ángel De la Vega Rodríguez
 * @brief Implementation for Editor class
 */

#include "Editor.h"

bool Editor::SaveFile(const std::string &fileName) {
    std::ofstream outputStream(fileName);
    if (!outputStream.is_open()){
        std::cerr << "Error opening file " << fileName << std::endl;
        return false;
    }

    std::stringstream saveBuffer; // String to be saved
    for (sf::Uint32 c : this->buffer){
        saveBuffer << c;
    }

    outputStream << saveBuffer.str();
    outputStream.close();

    this->documentChanged = false;
    return true;
}

bool Editor::HasChanged() const {
    return documentChanged;
}

int Editor::GetNumLines() {
    return this->lineBuffer.size();
}

void Editor::SwapLines(int lineA, int lineB) {
    if (lineA == lineB){
        return;
    }
    //TODO: Finish this part
}

sf::String Editor::GetLine(int lineNumber) {
    int bufferStart = this->lineBuffer[lineNumber];
    int nextBufferStart = this->lineBuffer[lineNumber-1];
    int numChars = nextBufferStart - bufferStart -1;
    return this->buffer.substring(bufferStart,numChars);
}


