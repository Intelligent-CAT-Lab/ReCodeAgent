/*
 * gapbuffer.h
 * Defines the interface for working with the gapbuffer data structure.
 * Note that the gapbuffer datatype should not be manually populated.
 * Use the interface methods for creating/destroying the structure.
 *
 * */

#ifndef TED_GAP_H
#define TED_GAP_H

#define MEM_ERROR 128

/*
 * Gap Buffer Data structure
 * A buffer that uses a "gap" within a string to allow addition of new characters to it.
 * The length of the buffer is the length of the gap + the length of the string
 *
 * Gap Buffer implements the buffer interface defined in base.h
 *
 * Implementation:
 * - The string is seen as a separate object starting from buffer[0] and extending str_len characters.
 * - The gap is always somewhere between this string, or at the prefix or suffix.
 * - The length of the buffer is always the sum of the string length and the gap length.
 * - The string is buffer[0:gap_loc-1] + buffer[(gap_loc+gap_len):(gap_loc+gap_len) + (str_len-gap_loc)]:
 *      - 0 to gap_loc-1 is the string up to the gap
 *      - (gap_loc+gap_len) is the index right after the gap
 *      - (gap_loc+gap_len) + (str_len-gap_loc-1) is the last index of the string.
 * */

typedef struct GapBuffer {
    char* buffer;  // Buffer containing string and gap
    int str_len;    // Length of the string
    int gap_len;    // Length of the gap
    int gap_loc;    // Gap location as an offset from the start of the buffer
} GapBuffer;



/*
 * INTERFACES
 * */

/*
 * Creates and initializes a new gap buffer. Initially the gap is of size `capacity`
 * returns NULL on fail.
 * */
GapBuffer* CreateGapBuffer(int capacity);


/*
 * DestroyGapBuffer Safely deallocates a gapbuffer. The pointer given is set to NULL after.
 * Passing a NULL pointer does nothing.
 * buffer: pointer to an allocated gapbuffer.
 * */
void DestroyGapBuffer(GapBuffer* instance);


/*
 * Inserts a character into the gap buffer. Resizes the buffer and the gap if necessary.
 * Returns 0 if successful or MEM_ERROR
 * */
int GapBufferInsertChar(GapBuffer* instance, char ch);


/*
 * Backspace deletes a character from the prefix of the gap, extending the gap length.
 * */
void GapBufferBackSpace(GapBuffer* instance);


/*
 * Moves the gap to a new location in the string. If the location is greater than str_len,
 * it moves to the end of the string. if less than 0, moves to the start of the string
 * */
int GapBufferMoveGap(GapBuffer* instance, int location);


/*
 * GetString returns a pointer to an allocated copy of the current string in the gapbuffer.
 * */
char* GapBufferGetString(GapBuffer* instance);


/*
 * Splits the buffer at the cursor location, returning a new buffer of the same capacity as the original
 * with the second half of the string. The original buffer will contain the first half of the string,
 * with the gap extending to fill the capacity.
 *
 * return a pointer to the new buffer, or NULL on failure.
 * */
GapBuffer* GapBufferSplit(GapBuffer* instance);


/*
 * Create a gap buffer containing the contents of str. The gap length is initialized to 'gap_len'.
 * The gap is always positioned at the end of the string.
 * if str is invalid, defaults to CreateGapBuffer(gap_len);
 *
 * returns an initialized GapBuffer* or NULL on error
 * */
GapBuffer* CreateGapBufferFromString(char* str, int gap_len);


/*
 * Given an index i, return the character at the location i.
 * The gap is ignored; acts similar to string index.
 * domain for i = [0, strlen-1]
 * returns null byte if the instance is invalid, the string is empty, or i is out of domain
 * */
char GapBufferCharAt(GapBuffer* instance, int i);


#endif //TED_GAP_H
