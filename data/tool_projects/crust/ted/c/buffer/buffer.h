//
// Created by Anfernee Jervis on 7/1/22.
//

#ifndef TED_BUFFER_H
#define TED_BUFFER_H

#include "gap.h"
#include <stdio.h>

#define DEFAULT_CAPACITY 100
#define DEFAULT_GAP_BUF_CAP 100

/*
 * TextBuffer
 * This data structure represents the current buffer of the text editor.
 * It's essentially an array of gap buffers, each gap buffer representing a line in the file.
 *
 * [] -> [contents of line |       |  one]
 * [] -> [contents of line |       |  twp]
 * [] -> [contents of line |       |  three]
 * [] -> [contents of line |       |  four]
 * [] -> [contents of line |       |  five]
 * [] -> [|                              |]    Blank line (with a full buffer)
 * [] -> NULL                                    No line. Signals end of the buffer
 * ...
 * [] -> NULL                                    Last element of the array. Signals end of capacity.
 *
 * Note that blank lines will have a buffer. This can be optimized later (with a performance penalty for inserts)
 * Array slots with a NULL pointer are non-existent lines, and signal the end of a file.
 *
 * Having the rows be an array allows constant lookup so file exploration is cheap.
 * Copying small strings is cheap so gap buffers for lines shouldn't be too expensive.
 *
 * Additionally, this structure will hold details about the current state of the text editor,
 * such as the cursor position (row, col).
 *
 * lines: array of GapBuffers representing the lines in a file.
 * lines_capacity: size of the lines array
 * cursorRow: row of the cursor
 * cursorCol: column of the cursor
 * cursorColMoved: whether the cursorCol changed (by a move operation for example)
 * last_line_loc: the last line in the buffer
 * */

typedef struct TextBuffer {
    GapBuffer** lines;
    int lines_capacity;
    int cursorRow;
    int cursorCol;
    int cursorColMoved;    // if cursorColMoved, a move must be performed on the gap buffer before inserts
    int last_line_loc;  // Index of the last line (also the number of lines)
} TextBuffer;


/*
 * CreateTextBuffer creates and initializes a new text buffer.
 * lines: The number of lines to support initially
 * line_size: Default initial line size to allocate.
 * Returns a pointer to an initialized TextBuffer or NULL
 * */
TextBuffer* CreateTextBuffer(int lines, int line_size);


/*
 * DestroyTextBuffer deallocates the structures in the TextBuffer, and the TextBuffer itself
 * */
void DestroyTextBuffer(TextBuffer* instance);

/*
 * MoveCursor moves the cursor to row and column given. If the values are out of bounds, it's moved
 * to the closest valid position (e.g. if row is negative, it's moved to row 0)
 *
 * Memory is allocated as needed.
 *
 * Returns 0 on success or MEM_ERROR
 * */

void TextBufferMoveCursor(TextBuffer* instance, int row, int col);


/*
 * Insert inserts a character at the cursor location. Space is made for the inserted character; it doesn't overwrite
 * a character that might be at the cursor's current position.
 *
 * Returns 0 on success, or MEM_ERROR
 * */

int TextBufferInsert(TextBuffer* instance, char ch);


/*
 * Backspace deletes the character that appears before the cursor location. Similar to hitting the backspace button.
 * */
int TextBufferBackspace(TextBuffer* instance);


/*
 * NewLine adds a new line to the buffer and moves the cursor to the start of that new line.
 * Handles the logic of hitting the return key.
 * Only works when at the end of the last line of the fine. Used to allow users to add more lines to the end of
 * the buffer
 *
 * e.g.
 * [] -> [contents of line |       |  one]
 * [] -> [contents {}of line |       |  twp]
 * [] -> [contents of line |       |  three]
 * In the TextBuffer above, {} is the cursor. What happens if Enter is hit at that location?
 * 1. The line needs to be split into two lines
 * 2. The new line needs to be placed immediately after the line that was split.
 * 3. The lines below need to be shifted down once
 *
 * The results of this:
 * [] -> [contents of line |       |  one]
 * [] -> [contents|                  |]
 * [] -> [{}|                  |of line twp]
 * [] -> [contents of line |       |  three]
 *
 * Returns 0 or MEM_ERROR
 * */
int TextBufferNewLine(TextBuffer* instance);


/*
 * Returns an allocated string of the contents of the line at the given index.
 * if index is out of bounds, or there was a memory error, returns NULL
 *
 * */
char* TextBufferGetLine(TextBuffer* instance, int row);



/*
 * Creates a TextBuffer with the contents of the file pointed to by the file pointer given.
 * By default, the gap buffer for each line is double the line size or DEFAULT_GAP_BUF_CAP, whichever is greater
 * If fp is NULL, behaves the same as CreateTextBuffer(DEFAULT_CAPACITY, DEFAULT_GAP_BUF_CAP),
 * returns NULL if there's an error, otherwise an initialized TextBuffer*
 * */
TextBuffer* CreateTextBufferFromFile(FILE* fp);

#endif //TED_BUFFER_H
