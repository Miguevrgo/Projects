/**
 * @file Cursor.h
 * @author Miguel Ángel De la Vega Rodríguez
 * @brief This file contains the declaration of the Cursor class used to
 * represent the position of the cursor in the text editor.
 */

#ifndef CODE_EDITOR_CURSOR_H
#define CODE_EDITOR_CURSOR_H


class Cursor {
public:
    Cursor();
    Cursor(int lineNumber, int charNumber);

    void MoveUp(int numLines = 1);
    void MoveDown(int numLines = 1);
    void MoveRight(int numChars = 1);
    void MoveLeft(int numChars = 1);

    void MoveEnd();
    void MoveStart();

    int GetLineNumber() const;
    int GetCharNumber() const;

    void SetPosition(int verticalPos, int horizontalPos);

private:
    int lineNumber;
    int charNumber;

    void UpdatePos(int verticalPos, int horizontalPos);
};


#endif //CODE_EDITOR_CURSOR_H
