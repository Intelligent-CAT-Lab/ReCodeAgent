//
// Created by Anfernee Jervis on 8/27/22.
//


void screen_append(const char *str, int size);


typedef struct Cursor {
    int x;
    int y;
} Cursor;


struct VirtualScreen {
    char* buffer;           // Contents on the screen (text & escape codes)
    int buf_pos;
    int len;
    Cursor cursor;
    int width;
    int height;
    int render_start_line;
};

/*
 * Returns the number of screen rows required to print a line of the given length.
 * undefined for line_length < 0
 * */
int required_screen_rows(int line_length, int screen_width) {

    // lines required is the number of times the screen width is filled len/width + 1 if there's a remainder
    if (line_length == 0) {
        return 1;

    } else {
        return (line_length / screen_width) + ((line_length % screen_width) > 0);
    }
}

/*
 * TODO: I can retroactively calculate this in the logic in the function that moves the cursor.
 * TODO: If i want to jump to file lines, I can then just call the function that moves the cursor x times
 *
 * TODO: After the cursor is moved to a new line, I can calculate what position the render start line needs
 * to be to keep the cursor visible. If cursor is above it, shift upwards. If below, shift downwards, ensuring enough
 * space for the cursor to be visible.
 *
 * Determines whether the cursor is off the screen, if so, shifts the display part of the buffer until
 * the cursor is back in view.
 * */
void move_cursor_in_view(TextBuffer* buffer, struct VirtualScreen* screen){

    Cursor buffer_cursor = {buffer->cursorRow, buffer->cursorCol};

    int cumul_req_rows = 0;
    int cur_line_required_rows;
    int cur_line = screen->render_start_line;


    // The entire render_start_line is guaranteed to be rendered (except the corner case where
    // its larger than the screen dimensions; a case we'll ignore for now)
    if (buffer_cursor.x < screen->render_start_line){
        screen->render_start_line = buffer_cursor.x;
    }

    // We'll calculate the current range in the buffer that is visible
    else {
        while (cur_line <= buffer->last_line_loc) {

            cur_line_required_rows = required_screen_rows(buffer->lines[cur_line]->str_len, screen->width);

            if ((cur_line_required_rows + cumul_req_rows) > screen->height - 1) {
                cur_line--;
                break;
            }

            cumul_req_rows += cur_line_required_rows;
            cur_line++;
        }

        // If the cursor is not in view, shift the text displayed until it is
        if (buffer_cursor.x > cur_line) {

            int rows_required = 0;

            // Calculate required space
            while(cur_line <= buffer_cursor.x){
                rows_required += required_screen_rows(buffer->lines[cur_line]->str_len, screen->width);
                cur_line++;
            }


            // shift the render start line down until we've made enough room
            while (rows_required > 0){
                rows_required -= required_screen_rows(buffer->lines[screen->render_start_line]->str_len, screen->width);
                screen->render_start_line++;

            }
        }
    }
}


void draw_editor_window(TextBuffer* buffer, struct VirtualScreen* screen){
    char* line = NULL;
    int cur_line = screen->render_start_line;
    int lines_written = 0;
    int screen_cols;

    while (cur_line <= buffer->last_line_loc && lines_written < screen->height - 1){

        screen_cols = screen->width;

        // Let's draw cur_line using as many screen rows as needed.
        line = TextBufferGetLine(buffer, cur_line);

        if (line == NULL){
            panic("draw editor cant get text of current line in buffer");
        }

        // Draw the line
        if (strlen(line) > screen_cols) {
            // Here we need multiple screen rows to draw 'line' we'll use whatever is available to draw it

            int i=0;
            int len_to_write;

            do {
                // if remaining line can fit in screen space, write the remaining line, else fill the remaining space
                len_to_write = screen_cols < strlen(&line[i]) ? screen_cols : strlen(&line[i]);

                screen_append(&line[i], len_to_write);
                screen_append("\r\n", 2); screen_append("\x1b[K", 3);
                i += len_to_write;
                lines_written++;

                // Screen is full, lets stop
                if (lines_written == screen->height - 2){
                    break;
                }

            } while (i < strlen(line) - 1);

        } else {
            screen_append(line, strlen(line));
            screen_append("\r\n", 2);
            lines_written++;
        }

        free(line);
        cur_line++;
    }

    // If there's remaining space, fill with blanks
    for (; lines_written < screen->height-2; lines_written++){
        screen_append("\r\n", 2);
    }
}


void set_virtual_cursor_position(TextBuffer* buffer, struct VirtualScreen* screen){

    int current_line = screen->render_start_line;
    int virtual_cursor_row = 1;
    int required_rows;

    while (current_line != buffer->cursorRow){
        required_rows = required_screen_rows(buffer->lines[current_line]->str_len, screen->width);

        virtual_cursor_row += required_rows;
        current_line++;
    }

    // if the cursor line wraps, we need to shift the cursor down the number of times it wraps
    virtual_cursor_row += (buffer->cursorCol) / screen->width;

    // now lets set the screen cursor x and y position
    screen->cursor.x = virtual_cursor_row;
    screen->cursor.y = (buffer->cursorCol % screen->width) + 1;
}
