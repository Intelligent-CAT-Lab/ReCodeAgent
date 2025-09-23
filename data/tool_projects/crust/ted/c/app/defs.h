//
// Created by Anfernee Jervis on 9/5/22.
//

#ifndef TED_DEFS_H
#define TED_DEFS_H

// ANSI Escape Codes
#define ESC '\x1b'
#define INVERT_COLOUR "\x1b[7m"
#define INVERT_COLOUR_SIZE 4
#define RESET_STYLE_COLOUR "\x1b[0m"


// CTRL_KEY macro returns the value of the k as a control key combination; basically CTRL + k
#define CTRL_KEY(k) ((k) & 0x1f)

void panic(const char* message);

#endif //TED_DEFS_H
