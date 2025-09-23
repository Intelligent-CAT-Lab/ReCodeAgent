#include <libgen.h>
#include <string.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <termios.h>
#include <sys/ioctl.h>
#include <sys/errno.h>
#include <stdbool.h>
#include "../buffer/buffer.h"
#include "defs.h"

#include "visual.c"

/* Constants */
enum specialKeys {
    BACKSPACE = 127,
    ARROW_UP = 1000,
    ARROW_DOWN,
    ARROW_LEFT,
    ARROW_RIGHT,
    PAGE_UP,
    PAGE_DOWN,
    HOME_KEY,
    END_KEY,
    DEL_KEY
};

/* structs */

// Main state & buffers
struct EditorState {

    // configuration states
    struct termios orig_termios;

    // File states
    char* file_name;
    char* file_path;         // path to file.
    bool flushed;             // was the most recent changes flushed to disk

    // buffer states
    TextBuffer* current_buffer;

    // Window states
    struct VirtualScreen screen;
};

/* global editor state */
struct EditorState editor_state;


/* prototypes */

/* System and Configuration */
void initialize(int argc, char* argv[]);
void cleanup();
void set_window_size();
void disableRawMode();
void enableRawMode();

/* Screen Manipulation */
void render_screen();
void draw_screen();
void draw_status_line(int line_size);


/* Cursor Movement */
void up_arrow();
void down_arrow();
void right_arrow();
void left_arrow();

/* Input */
int read_char();
void process_keypress();

/* File Manipulation*/
int flush_buffer_to_file();
int load_file_and_initialize_buffer();


/* Main */
int main(int argc, char* argv[]) {
    initialize(argc, argv);

    while (1) {
        draw_screen();
        render_screen();
        process_keypress();
    }
}


/******************************* Implementations *********************************/

void initialize(int argc, char* argv[]){

    // Get file path information
    // TODO: Dont attempt to load file if no path is given.
    if (argc >= 2){
        editor_state.file_path = argv[1];
    }
    else {
        editor_state.file_path = "Empty Buffer";
    }
    editor_state.file_name = basename(editor_state.file_path);

    // Loads the file and initialize the textbuffer
    load_file_and_initialize_buffer();

    // initialize screen
    enableRawMode();
    set_window_size();

    // double the buffer for the screen to allow escape codes to be sent without overflowing
    editor_state.screen.len = editor_state.screen.height * editor_state.screen.width * sizeof(char) * 2;
    editor_state.screen.buffer = malloc(editor_state.screen.len);
    editor_state.screen.buf_pos = 0;

    // the line the screen starts printing from
    editor_state.screen.render_start_line = 0;

    editor_state.flushed = true;
}

/*
 * Returns:
 * -1: If there was an issue with opening the file; the buffer is still initialized as a blank buffer
 * MEM_ERROR if it wasn't successfully loaded, or there was an issue resizing/manipulating/initializing the TextBuffer.
 * */
int load_file_and_initialize_buffer() {

    // We'll open all files in read mode. If there's no file, we'll just have a blank buffer.
    // Only when writing to file, will we rewrite or create + write to the file.
    FILE* fp = fopen(editor_state.file_path, "r");

    // CreateTextBufferFromFile handles NULL values so we can just pass editor_state.fp and check the return
    editor_state.current_buffer = CreateTextBufferFromFile(fp);

    if (editor_state.current_buffer == NULL){
        return MEM_ERROR;
    }

    if (fp == NULL){
        return -1;
    }

    fclose(fp);
    return 0;
}

void cleanup(){

    // Clear screen
    write(STDOUT_FILENO, "\x1b[2J", 4);
    write(STDOUT_FILENO, "\x1b[H", 3);

    // free memory for screen
    free(editor_state.screen.buffer);

    // Free the text buffer
    DestroyTextBuffer(editor_state.current_buffer);
}

void panic(const char* message){
    // Clear screen
    write(STDOUT_FILENO, "\x1b[2J", 4);
    write(STDOUT_FILENO, "\x1b[H", 3);
    // Enable cursor
    write(STDOUT_FILENO,"\x1b[?25h", 6);
    perror(message);
    exit(1);
}

void set_window_size() {
    struct winsize ws;

    if (ioctl(STDOUT_FILENO, TIOCGWINSZ, &ws) == -1 || ws.ws_col == 0) {
        // panic("Failed to get window size");

        // manually get window size
        if (write(STDOUT_FILENO, "\x1b[999C\x1b[999B", 12) != 12) {
            panic("Failed to get window size");
        }

        // read the response
        char buf[32];
        for (int i = 0; i < sizeof(buf); i++) {
            if (read(STDIN_FILENO, &buf[i], 1) != 1) {
                panic("Failed to get window size");
            }

            if (buf[i] == 'R') {
                buf[i + 1] = '\0';
                break;
            }
        }

        // parse the response and set the windows size
        if (sscanf(buf, "%d;%d", &editor_state.screen.height, &editor_state.screen.width) != 2) {
            panic("Failed to get windows size");
        }

    } else {
        editor_state.screen.height = ws.ws_row;
        editor_state.screen.width = ws.ws_col;
    }
}

void disableRawMode() {
    if (tcsetattr(STDIN_FILENO, TCSAFLUSH, &editor_state.orig_termios) == -1) {
        panic("tcsetattr");
    }
}

// Test raw mode
void enableRawMode(){

    if(tcgetattr(STDIN_FILENO, &editor_state.orig_termios) == -1) {
        panic("tcgetattr");
    }

    atexit(disableRawMode);

    struct termios raw = editor_state.orig_termios;

    /*
     * Local modes:
     * Here we are disabling:
     * ECHO: don't echo input characters back to the screen.
     * ICANON: canonical mode. Now each character is read as soon as it is typed.
     * ISIG: SIGINT/SIGSTOP signal interpretation. Now Ctrl-C/Ctrl-Z won't send signals to the process.
     * IEXTEN: extended input processing. Now Ctrl-V/Ctrl-C/Ctrl-D won't be interpreted as special characters.
     *
     * | - logic OR
     * & - logic AND
     * ~ - logic XOR
     */
    raw.c_lflag &= ~(ECHO | ICANON | ISIG | IEXTEN);

    /*
     * Input modes:
     * Here we're disabling:
     * IXON: disable software flow control.
     * ICRNL: maps CR to NL (otherwise, it will be CR+NL). Now Ctrl-M will be ignored.
     * BRKINT: disable break processing. Now break condition will be ignored.
     * INPCK: disable parity checking. Now parity errors will be ignored. (This doesn't matter anymore)
     * ISTRIP: disable stripping of eighth bit. Now eighth bit will be ignored. (This probably doesn't matter)
     **/
    raw.c_iflag &= ~(ICRNL | IXON | BRKINT | INPCK);

    /*
     * Output modes:
     * Here we're disabling:
     * OPOST: output processing. Now output processing is disabled.
     *
     **/
    raw.c_oflag &= ~(OPOST);

    /*
     * Control modes:
     * Here we're disabling:
     * CS8: 8-bit characters. Now 8-bit characters will be used.
     **/
    raw.c_cflag |= (CS8);

    /*
     * c_cc - control characters.
     * VMIN: minimum number of characters to read. Set to 0 so that we can read whatever is available.
     * VTIME: time to wait for characters to be available. Set to 1 (100ms).
     **/
    raw.c_cc[VMIN] = 0;
    raw.c_cc[VTIME] = 1;

    if (tcsetattr(STDIN_FILENO, TCSAFLUSH, &raw) == -1) {
        panic("tcsetattr");
    }
}


void screen_append(const char *str, int size) {

    if (editor_state.screen.buffer != NULL && (editor_state.screen.len - editor_state.screen.buf_pos) > size) {
        memcpy(editor_state.screen.buffer + editor_state.screen.buf_pos, str, size);
        editor_state.screen.buf_pos += size;
    }
}


/* Display */
void render_screen() {

    // flush internal screen to display
    write(STDOUT_FILENO, editor_state.screen.buffer, strlen(editor_state.screen.buffer));
}


void draw_screen(){

    editor_state.screen.buf_pos = 0; // reset the screen

    // Disable cursor
    screen_append("\x1b[?25l", 6);

    // Clear screen
    screen_append("\x1b[2J", 4);

    // Move cursor to the top
    screen_append("\x1b[H", 3);

    move_cursor_in_view(editor_state.current_buffer, &editor_state.screen);
    draw_editor_window(editor_state.current_buffer, &editor_state.screen);
    draw_status_line(editor_state.screen.width);

    set_virtual_cursor_position(editor_state.current_buffer, &editor_state.screen);

    int row = editor_state.screen.cursor.x;
    int col = editor_state.screen.cursor.y;

    char buf[32];
    sprintf(buf, "\x1b[%d;%dH", row, col);

    screen_append(buf, strlen(buf));

    // Enable cursor
    screen_append("\x1b[?25h", 6);

    // End the string (so we can get strlen)
    screen_append("\0", 1);
}


void draw_status_line(int line_size) {

    const char commands[] = "Ctrl+Q-quit Ctrl+S-Save";
    unsigned int commands_len = sizeof commands-1;

    const char modified[] = "changed";
    int modified_len = sizeof modified-1;

    /*
     * {|file cursor space| |modified len|    |       controls       |
     * [filename.c | 5,50   changed           Ctrl+Q-quit Ctrl+S-Save]
     * */

    // Calculate space for each part of the status line
    int file_cursor_space = line_size - (commands_len + modified_len);
    int cur_col_digits = snprintf(NULL, 0, "%d", editor_state.current_buffer->cursorCol);
    int cur_row_digits = snprintf(NULL, 0, "%d", editor_state.current_buffer->cursorRow);

    int f_name_space = file_cursor_space - (cur_col_digits + cur_row_digits + 5);
    int file_name_size = strlen(editor_state.file_name);

    int cursor_info_buffer_size = file_cursor_space - f_name_space;
    char cursor_info_buffer[cursor_info_buffer_size];


    // invert the colours
    screen_append(INVERT_COLOUR, INVERT_COLOUR_SIZE);

    // Write file name. If its longer than available space, we'll cut it short with ellipsis
    if (file_name_size > f_name_space){
        screen_append(editor_state.file_name, f_name_space - 4);

        screen_append("... ", 4);

    } else {
        screen_append(editor_state.file_name, file_name_size);
    }

    // Write col and row info
    sprintf(cursor_info_buffer,
            " | %d,%d ",
            editor_state.current_buffer->cursorRow, editor_state.current_buffer->cursorCol);

    screen_append(cursor_info_buffer, strlen(cursor_info_buffer));

    // Indicate if buffer was modified since last write.
    if (!editor_state.flushed) {
        screen_append(modified, modified_len);
    }
    else {
        memset(editor_state.screen.buffer + editor_state.screen.buf_pos, ' ', modified_len);
        editor_state.screen.buf_pos += modified_len;
    }

    // Fill with whitespace
    memset(editor_state.screen.buffer + editor_state.screen.buf_pos, ' ', f_name_space - file_name_size);
    editor_state.screen.buf_pos += f_name_space - file_name_size;

    // print help
    screen_append(commands, commands_len);

    // Finally reset the colour and style
    screen_append(RESET_STYLE_COLOUR, INVERT_COLOUR_SIZE);
}



/*
 * Returns -1 when unable to open the file, -2 on write error, 0 on success
 * */
int flush_buffer_to_file(){

    FILE* fp = fopen(editor_state.file_path, "w");
    char* line = NULL;

    if (fp == NULL){
        return -1;
    }

    for (int i=0; i<=editor_state.current_buffer->last_line_loc; i++){
        line = TextBufferGetLine(editor_state.current_buffer, i);

        if (line == NULL){
            fclose(fp);
            return -2;
        }


        if (fputs(line, fp) < strlen(line)){
            fclose(fp);
            return -2;
        }
    }

    fclose(fp);
    return 0;
}


/* Input */
/*
 * read_char is heavily motivated by this tutorial: https://viewsourcecode.org/snaptoken/kilo/03.rawInputAndOutput.html
 * whose code comes from kilo: http://antirez.com/news/108
 * */
int read_char(){
    char c;
    ssize_t err;

    while((err = read(STDIN_FILENO, &c, 1)) != 1) {
        if (err == EAGAIN) {
            panic("read_char: read() returned EAGAIN");
        }
    }

    // Handle escape sequences
    if (c == ESC){
        char seq[3];

        if (read(STDIN_FILENO, &seq[0], 1) != 1) return ESC;
        if (read(STDIN_FILENO, &seq[1], 1) != 1) return ESC;

        if (seq[0] == '[') {
            if (seq[1] >= '0' && seq[1] <= '9') {
                if (read(STDIN_FILENO, &seq[2], 1) != 1) return ESC;
                if (seq[2] == '~') {
                    switch (seq[2]) {
                        case '1': return HOME_KEY;
                        case '3': return DEL_KEY;
                        case '4': return END_KEY;
                        case '5': return PAGE_UP;
                        case '6': return PAGE_DOWN;
                        case '7': return HOME_KEY;
                        case '8': return END_KEY;
                    }
                }
            } else {
                switch (seq[1]) {
                    case 'A': return ARROW_UP;
                    case 'B': return ARROW_DOWN;
                    case 'C': return ARROW_RIGHT;
                    case 'D': return ARROW_LEFT;
                    case 'H': return HOME_KEY;
                    case 'F': return END_KEY;
                }
            }
        }
        else if (seq[0] == 'O') {
            switch (seq[1]){
                case 'H': return HOME_KEY;
                case 'F': return END_KEY;
            }
        }

        return ESC;

    } else {
        return c;
    }
}


void process_keypress(){

    int c = read_char();
    int err;

    switch (c) {

        case '\r':
            TextBufferNewLine(editor_state.current_buffer);
            break;

        case ARROW_UP: up_arrow(); break;
        case ARROW_DOWN: down_arrow(); break;
        case ARROW_LEFT: left_arrow(); break;
        case ARROW_RIGHT: right_arrow(); break;

            // We wont use these keys for now
        case PAGE_UP:
        case PAGE_DOWN:
        case HOME_KEY:
        case END_KEY:
        case DEL_KEY:
            break;

        case CTRL_KEY('l'):
        case '\x1b':
            break;

            // Save buffer state to file
        case CTRL_KEY('s'):
            err = flush_buffer_to_file();
            editor_state.flushed = true;
            break;

        case CTRL_KEY('q'):
            cleanup();
            exit(0);
            break;

            // Backspace
        case BACKSPACE:
        case CTRL_KEY('h'):
            TextBufferBackspace(editor_state.current_buffer);
            editor_state.flushed = false;
            break;
        default:
            TextBufferInsert(editor_state.current_buffer, c);
            editor_state.flushed = false;
            break;
    }
}


void up_arrow() {

    int col = editor_state.current_buffer->cursorCol;
    int row = editor_state.current_buffer->cursorRow;

    if (row > 0){
        row--;

        TextBufferMoveCursor(editor_state.current_buffer, row, col);
    }

    // ding the terminal if you figure out how to
}

void down_arrow() {

    int col = editor_state.current_buffer->cursorCol;
    int row = editor_state.current_buffer->cursorRow;

    if (row < editor_state.current_buffer->last_line_loc){
        row++;

        TextBufferMoveCursor(editor_state.current_buffer, row, col);
    }

    // ding terminal
}

void left_arrow() {
    int row = editor_state.current_buffer->cursorRow;
    int col = editor_state.current_buffer->cursorCol - 1;
    TextBufferMoveCursor(editor_state.current_buffer, row, col);
}

void right_arrow() {
    int row = editor_state.current_buffer->cursorRow;
    int col = editor_state.current_buffer->cursorCol + 1;
    TextBufferMoveCursor(editor_state.current_buffer, row, col);
}