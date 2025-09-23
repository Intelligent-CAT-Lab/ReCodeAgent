//
// Created by Anfernee Jervis on 7/1/22.
//

#include "buffer.h"
#include "gap.h"
#include <stdlib.h>
#include <strings.h>


TextBuffer* CreateTextBuffer(int num_lines, int line_size){
    TextBuffer* textBuffer = malloc(sizeof(TextBuffer));

    if (textBuffer == NULL){
        return NULL;
    }

    // Allocate gap buffer array
    textBuffer->lines = malloc(sizeof(GapBuffer*) * num_lines);

    if (textBuffer->lines == NULL){
        return NULL;
    }

    // allocate the first line
    textBuffer->lines[0] = CreateGapBuffer(line_size);

    if (textBuffer->lines[0] == NULL){
        return NULL;
    }

    // NULL the rest of the lines
    for (int i=1; i<num_lines; i++){
        textBuffer->lines[i] = NULL;
    }

    textBuffer->lines_capacity = num_lines;
    textBuffer->cursorRow = 0;
    textBuffer->cursorCol = 0;
    textBuffer->cursorColMoved = 0;
    textBuffer->last_line_loc = 0;

    return textBuffer;
}


void DestroyTextBuffer(TextBuffer* instance){

    // Deallocate each GapBuffer
    for(int i=0; i<instance->lines_capacity; i++){

        // Break if we're at the last line
        if (instance->lines[i] == NULL){
            break;
        }

        DestroyGapBuffer(instance->lines[i]);
    }

    // Deallocate the gapbuffer array and the TextBuffer itself
    free(instance->lines);
    free(instance);
}


void TextBufferMoveCursor(TextBuffer* instance, int row, int col){
    if (row > instance->last_line_loc){
        row = instance->last_line_loc;
    }
    if (row < 0){
        row = 0;
    }

    instance->cursorRow = row;

    if (col > instance->lines[row]->str_len){
        col = instance->lines[row]->str_len;
    }

    if (col < 0){
        col = 0;
    }

    // We need to indicate that the column moved so the gap can be moved before inserting
    if (instance->cursorCol != col){
        instance->cursorColMoved = 1;
    }

    instance->cursorCol = col;
}


int TextBufferInsert(TextBuffer* instance, char ch){

    int err;

    // If the cursor column changed, we need to move the gap buffer before inserting
    if (instance->cursorColMoved) {
        err = GapBufferMoveGap(instance->lines[instance->cursorRow], instance->cursorCol);

        if (err != 0){
            return err;
        }

        instance->cursorColMoved = 0;
    }

    err = GapBufferInsertChar(instance->lines[instance->cursorRow], ch);

    if (err != 0){
        return err;
    }

    instance->cursorCol = instance->lines[instance->cursorRow]->gap_loc;
    return 0;
}


int TextBufferBackspace(TextBuffer* instance){
    int err;

    // If the cursor column changed, we need to move the gap buffer before deleting
    if (instance->cursorColMoved) {
        err = GapBufferMoveGap(instance->lines[instance->cursorRow], instance->cursorCol);

        if (err != 0){
            return err;
        }

        instance->cursorColMoved = 0;
    }

    GapBufferBackSpace(instance->lines[instance->cursorRow]);
    instance->cursorCol = instance->lines[instance->cursorRow]->gap_loc;

    return 0;
}


int TextBufferNewLine(TextBuffer* instance){
    // split the current GapBuffer where the gap is.
    // Create a new GapBuffer and copy the second half of the string to the new GapBuffer
    // Check if there's space to add a new line. If not, reallocate `lines` and copy the old to the new
    // Finally, check if there's an allocated line below the current line. If there is, shift the lines array
    // down one place with memmove.

    int errno;

    // First ensure the gap location reflects the cursor position
    if (instance->cursorColMoved){
        errno = GapBufferMoveGap(instance->lines[instance->cursorRow], instance->cursorCol);

        if (errno != 0){
            return errno;
        }

        instance->cursorColMoved = 0;
    }

    // Split the current GapBuffer at the gap location
    // New gap will have the gap at the start of its string.
    GapBuffer* newline = GapBufferSplit(instance->lines[instance->cursorRow]);

    if (newline == NULL){
        return MEM_ERROR;
    }

    // Check if there's space to add a new line. If not, reallocate `lines` and copy the old to the new
    if (instance->last_line_loc == instance->lines_capacity - 1){
        GapBuffer** new_lines = realloc(instance->lines, sizeof(GapBuffer*) * (instance->lines_capacity * 2));

        if (new_lines == NULL){
            // TODO: Panic here. recovering at the moment is hard
            return MEM_ERROR;
        }

        instance->lines = new_lines;
        instance->lines_capacity *= 2;
    }

    // Shift the lines array down one place with memmove
    memmove(instance->lines + instance->cursorRow + 2,
            instance->lines + instance->cursorRow + 1,
            sizeof(GapBuffer*) * (instance->last_line_loc - instance->cursorRow));


    // Set the new line to the next available slot
    instance->lines[instance->cursorRow + 1] = newline;

    // Update the last line location
    instance->last_line_loc++;

    // Update the cursor position
    instance->cursorRow++;
    instance->cursorCol = newline->gap_loc;

    return 0;
}

char *TextBufferGetLine(TextBuffer *instance, int row) {
    if (row > instance->last_line_loc){
        return NULL;
    }
    if (row < 0){
        return NULL;
    }

    return GapBufferGetString(instance->lines[row]);
}

TextBuffer* CreateTextBufferFromFile(FILE* fp){

    TextBuffer* new_tbuffer = CreateTextBuffer(DEFAULT_CAPACITY, DEFAULT_GAP_BUF_CAP);

    if (new_tbuffer == NULL) {
        return NULL;
    }

    if (fp == NULL){
        return new_tbuffer;
    }

    // Delete the first line. Reset the cursor and last line positions
    DestroyGapBuffer(new_tbuffer->lines[new_tbuffer->last_line_loc]);
    new_tbuffer->last_line_loc = -1;

    // Read line and create gap buffer from the line, then append it to the text buffer. update last line pos
    char *line = NULL;
    size_t len = 0;
    size_t read;
    int line_gap_size;

    while ((read = getline(&line, &len, fp)) != -1 ) {

        // We dont want to include the newline character
        if (line[read-1] == '\n') {
            line[read - 1] = '\0';
        }

        // reallocate if out of space
        if (new_tbuffer->last_line_loc == new_tbuffer->lines_capacity-1){
            new_tbuffer->lines = realloc(
                    new_tbuffer->lines, sizeof(GapBuffer*) * new_tbuffer->lines_capacity * 2);

            if (new_tbuffer->lines == NULL){
                DestroyTextBuffer(new_tbuffer);
                return NULL;
            }

            new_tbuffer->lines_capacity = new_tbuffer->lines_capacity * 2;
        }

        // Create a new gap buf with the read line, append it to the tbuffer and update the last line location

        // the gap size will be max(DEFAULT_GAP_BUF_CAP, read * 2)
        line_gap_size = read * 2 < DEFAULT_GAP_BUF_CAP ? DEFAULT_GAP_BUF_CAP : read * 2;

        new_tbuffer->lines[new_tbuffer->last_line_loc + 1] = CreateGapBufferFromString(line, line_gap_size);

        if (new_tbuffer->lines[new_tbuffer->last_line_loc + 1] == NULL){
            DestroyTextBuffer(new_tbuffer);
            return NULL;
        }

        new_tbuffer->last_line_loc++;
    }

    free(line);
    return new_tbuffer;
}