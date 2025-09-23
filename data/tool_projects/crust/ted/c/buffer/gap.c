//
// Created by Anfernee Jervis on 6/29/22.
//

#include <stdlib.h>
#include <string.h>

#include "gap.h"


/*
 * TODO: Consider rewriting this using realloc and memmove.
 * helper function for copying the contents of a GapBuffer.buffer to a new buffer, with/without
 * resizing the gap. This method is useful when moving the 'cursor' or when resizing a filled buffer.
 * instance: GapBuffer instance
 * new_capacity: The new size of the buffer. If <= original capacity, the same capacity will be used.
 *
 * return 0 on success or MEM_ERROR
 * */
int resizeBuffer(GapBuffer* instance, int new_capacity){

    int buffer_size = instance->str_len + instance->gap_len;

    if (new_capacity < buffer_size){
        new_capacity = buffer_size;
    }

    // If we increase the capacity, all the new space should go to the gap.
    int gap_size = instance->gap_len + (new_capacity - buffer_size);

    // First malloc new buffer
    char* new_buffer = malloc(sizeof(char) * new_capacity);

    if (new_buffer == NULL){
        return MEM_ERROR;
    }

    // first, copy up to the start of the gap
    memcpy(new_buffer, instance->buffer, sizeof(char) * (instance->gap_loc));

    // Copy the rest of the buffer, starting from the suffix of the gap
    memcpy(
            (new_buffer + instance->gap_loc + gap_size),                // [a, a, a, a, _, _, _, starts here>a, a, a, a]
            (instance->buffer + instance->gap_loc + instance->gap_len), // [a, a, a, a, _, starts here>a, a, a, a]
            instance->str_len - instance->gap_loc);  //  now we copy the remaining str_len - gap_loc characters

    free(instance->buffer);
    instance->buffer = new_buffer;
    instance->gap_len = gap_size;

    return 0;
}


GapBuffer* CreateGapBuffer(int capacity){

    GapBuffer* gap_buffer = malloc(sizeof(GapBuffer));

    if (gap_buffer == NULL){
        return NULL;
    }

    gap_buffer->buffer = malloc(sizeof(char) * capacity);

    if (gap_buffer->buffer == NULL){
        free(gap_buffer);
        return NULL;
    }

    gap_buffer->gap_loc = 0;
    gap_buffer->gap_len = capacity;
    gap_buffer->str_len = 0;

    return gap_buffer;
}


void DestroyGapBuffer(GapBuffer * instance){
    free(instance->buffer);
    free(instance);
}


int GapBufferInsertChar(GapBuffer* instance, char ch){

    int errno;

    // If the gap is about to close, resize it.
    if (instance->gap_len <= 1){
        int current_cap = instance->gap_len + instance->str_len;
        if ((errno = resizeBuffer(instance, current_cap * 2)) != 0){
            return errno;
        }
    }

    instance->buffer[instance->gap_loc] = ch;
    instance->str_len++;
    instance->gap_loc++;
    instance->gap_len--;

    return 0;
}


void GapBufferBackSpace(GapBuffer* instance){

    if (instance->gap_loc > 0){
        instance->gap_loc--;
        instance->gap_len++;
        instance->str_len--;
    }
}


int GapBufferMoveGap(GapBuffer* instance, int location){
    if (location > instance->str_len){
        location = instance->str_len;
    }

    if (location < 0){
        location = 0;
    }

    int capacity = instance->gap_len + instance->str_len;
    char* new_buffer = malloc(sizeof(char) * capacity);

    if (new_buffer == NULL){
        return MEM_ERROR;
    }

    if (location < instance->gap_loc){

        // here the new location is before the current gap location
        // copy up to location
        memcpy(new_buffer, instance->buffer, sizeof(char) * (location));

        // then copy what remains up to instance.gap_loc, to the position after the gap in the new buffer
        memcpy(new_buffer + location + instance->gap_len,
               instance->buffer + location,
               sizeof(char) * (instance->gap_loc-location));

        // copy the rest of the string
        memcpy((new_buffer + location + instance->gap_len + (instance->gap_loc-location)),
                (instance->buffer + instance->gap_loc + instance->gap_len),
               sizeof(char) * instance->str_len - instance->gap_loc);  //  now we copy the remaining str_len - gap_loc characters
    } else {

        // here the new location is further after the current gap location
        // copy upto original gap
        memcpy(new_buffer, instance->buffer, sizeof(char) * (instance->gap_loc));

        // copy what's left before the new location
        memcpy(new_buffer+instance->gap_loc,
               instance->buffer + instance->gap_loc + instance->gap_len,
               sizeof(char) * (location - instance->gap_loc));

        // now copy the remaining string after the gap
        memcpy(new_buffer + location + instance->gap_len,
               instance->buffer + instance->gap_loc + instance->gap_len + (location - instance->gap_loc),
               sizeof(char) * instance->str_len - location);
    }

    free(instance->buffer);
    instance->buffer = new_buffer;
    instance->gap_loc = location;
    return 0;
}

char* GapBufferGetString(GapBuffer* instance){
    char* buffer = malloc(1 + (sizeof(char) * instance->str_len));

    if (buffer == NULL){
        return NULL;
    }

    // Copy before gap
    memcpy(buffer, instance->buffer, instance->gap_loc);

    // Copy after gap
    memcpy(buffer + instance->gap_loc,
           instance->buffer + instance->gap_loc + instance->gap_len,
           instance->str_len - instance->gap_loc);

    buffer[instance->str_len] = '\0';

    return buffer;
}



#pragma clang diagnostic push
#pragma ide diagnostic ignored "DanglingPointer"  // Ignore because CreateGapBuffer never returns a deallocated pointer.
GapBuffer* GapBufferSplit(GapBuffer *instance) {
    // split the current GapBuffer where the gap is.
    // Create a new GapBuffer and copy the second half of the string to the new GapBuffer

    int capacity = instance->gap_len + instance->str_len;
    int second_half_of_str_len = instance->str_len - instance->gap_loc;

    GapBuffer* new_gap_buffer = CreateGapBuffer(capacity);

    if (new_gap_buffer == NULL){
        return NULL;
    }

    // copy the second half of the string to the new GapBuffer
    // The string is copied to the location at the end of the gap in the new GapBuffer
    // |           gap             |second half of string
    memcpy(new_gap_buffer->buffer + (capacity - second_half_of_str_len),
           instance->buffer + instance->gap_loc + instance->gap_len,
           second_half_of_str_len);

    // set str_len, gap_loc and gap_len of the new GapBuffer
    new_gap_buffer->str_len = second_half_of_str_len;
    new_gap_buffer->gap_loc = 0;
    new_gap_buffer->gap_len = capacity - second_half_of_str_len;


    // set str_len, gap_len of the old GapBuffer
    instance->str_len = instance->gap_loc;
    instance->gap_len = capacity - instance->str_len;

    return new_gap_buffer;

}
#pragma clang diagnostic pop


#pragma clang diagnostic push
#pragma ide diagnostic ignored "DanglingPointer" // Ignore because CreateGapBuffer never returns a deallocated pointer.
GapBuffer* CreateGapBufferFromString(char* str, int gap_len){

    int s_len = strlen(str);
    int capacity = s_len + gap_len;
    GapBuffer* new_buffer;

    if (str == NULL || s_len == 0){
        return CreateGapBuffer(gap_len);

    } else {
        new_buffer = CreateGapBuffer(capacity);

        if (new_buffer == NULL){
            return NULL;
        }

        // Copy the string to the buffer
        memcpy(new_buffer->buffer, str, s_len);

        // update gap values
        new_buffer->str_len = s_len;
        new_buffer->gap_loc = s_len;
        new_buffer->gap_len = gap_len;

        return new_buffer;
    }
}

#pragma clang diagnostic pop


char GapBufferCharAt(GapBuffer *instance, int i) {

    if ( instance == NULL ||
         instance->str_len == 0 ||
         i < 0 || i >= instance->str_len){
        return '\0';
    }

    if (i < instance->gap_loc){
        return instance->buffer[i];
    }
    else {
        return instance->buffer[i + instance->gap_len];
    }
}