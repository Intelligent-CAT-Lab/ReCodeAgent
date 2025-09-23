//
// Created by Anfernee Jervis on 7/1/22.
//

#include <assert.h>
#include <stdlib.h>
#include "stdio.h"
#include <string.h>
#include "../buffer/gap.h"
#include "../buffer/buffer.h"


// Test Suites
void TestGapBuffer();
void TestTextBuffer();

FILE* test_fp;

/*
 * Contents of tests/runtests.txt
aaaaaaaaaaa\n
aaaaaaaaaa\n
aaaaaaaaa\n
 * */
char test_file_path[] = "tests/runtests.txt";


int main(){

    if ((test_fp = fopen(test_file_path, "r")) == NULL){
        printf("Error: Failed to open test file. Please run the test in the project directory.\n'"
               "tests/runtests.c' must be valid in the working directory");

        exit(1);
    }

    TestGapBuffer();
    TestTextBuffer();
    printf("All tests passed!\n");
}


void string_comp_assert(char* str1, const char* str2){
    assert(str1 != NULL);
    assert(str2 != NULL);
    assert(strcmp(str1, str2) == 0);
    free(str1);
}

void TestGapBuffer(){
    printf("\n\nTesting GapBuffer\n");

    GapBuffer* buffer = CreateGapBuffer(20);
    assert(buffer != NULL);

    char* string_holder = NULL;
    char* string_holder2 = NULL;
    int err;

    const char sample1[] = "";
    const char sample2[] = "a";
    const char sample3[] = "aaaaaaaaaaa";  // 11 characters
    const char sample4[] = "aaaaaaaaaaaaaaaaaaaaa";  // 21 characters
    const char sample5[] = "aaaaaaaaaaaaaaaaaaaa";  // 20 characters
    const char sample6[] = "aaabbbaaaaaaaaaaaaaaaaa";
    const char sample7[] = "cccaaabbbaaaaaaaaaaaaaaaaa";
    const char sample8[] = "cccaaabbbaaaaaaaaaaaaaaaaaddd";
    const char sample9[] = "cccaaa";
    const char sample10[] ="bbbaaaaaaaaaaaaaaaaaddd";
    const char sample11[] = "cccaaax";
    const char sample12[] ="ybbbaaaaaaaaaaaaaaaaaddd";
    const char sample13[] ="ybbbaaaaaaaaaaaaaaaaadddp";


    printf("Test 1, empty string\n");
    string_holder = GapBufferGetString(buffer);
    string_comp_assert(string_holder, sample1);


    printf("Test 2 insert char\n");
    err = GapBufferInsertChar(buffer, 'a');
    assert(err == 0);
    string_holder = GapBufferGetString(buffer);
    string_comp_assert(string_holder, sample2);


    printf("Test 2.1 insert char\n");
    for (int i=0; i<10; i++){
        err = GapBufferInsertChar(buffer, 'a');
        assert(err == 0);
    }
    string_holder = GapBufferGetString(buffer);
    string_comp_assert(string_holder, sample3);


    printf("Test 2.2 insert char (resize) \n");
    for (int i=0; i<10; i++){
        err = GapBufferInsertChar(buffer, 'a');
        assert(err == 0);
    }
    string_holder = GapBufferGetString(buffer);
    string_comp_assert(string_holder, sample4);


    printf("Test 3 Backspace \n");
    GapBufferBackSpace(buffer);
    string_holder = GapBufferGetString(buffer);
    string_comp_assert(string_holder, sample5);


    printf("Test 4 GapBufferMoveGap, TextBufferInsert and Backspace \n");

    err = GapBufferMoveGap(buffer, 3);
    assert(err == 0);

    for (int i=0; i<3; i++) {
        err = GapBufferInsertChar(buffer, 'b');
        assert(err == 0);
    }
    string_holder = GapBufferGetString(buffer);
    string_comp_assert(string_holder, sample6);


    printf("Test 4.1 GapBufferMoveGap, TextBufferInsert and Backspace \n");

    err = GapBufferMoveGap(buffer, 0);
    assert(err == 0);

    for (int i=0; i<3; i++) {
        err = GapBufferInsertChar(buffer, 'c');
        assert(err == 0);
    }
    string_holder = GapBufferGetString(buffer);
    string_comp_assert(string_holder, sample7);


    printf("Test 4.2 GapBufferMoveGap, TextBufferInsert and Backspace \n");

    err = GapBufferMoveGap(buffer, buffer->str_len);
    assert(err == 0);

    for (int i=0; i<3; i++) {
        err = GapBufferInsertChar(buffer, 'd');
        assert(err == 0);
    }
    string_holder = GapBufferGetString(buffer);
    string_comp_assert(string_holder, sample8);


    printf("Test 5 Split\n");
    err = GapBufferMoveGap(buffer, 6);
    assert(err == 0);

    GapBuffer* buffer2 = GapBufferSplit(buffer);
    assert(buffer2 != NULL);

    string_holder = GapBufferGetString(buffer);
    string_holder2 = GapBufferGetString(buffer2);

    string_comp_assert(string_holder, sample9);
    string_comp_assert(string_holder2, sample10);

    printf("Test 5.1 Split\n");
    err = GapBufferInsertChar(buffer, 'x');
    assert(err == 0);

    string_holder = GapBufferGetString(buffer);
    string_comp_assert(string_holder, sample11);

    printf("Test 5.2 Split\n");

    err = GapBufferInsertChar(buffer2, 'y');
    assert(err == 0);

    string_holder = GapBufferGetString(buffer2);
    string_comp_assert(string_holder, sample12);


    printf("Test 6 Create From string\n");

    GapBuffer* buffer3 = CreateGapBufferFromString((char *)sample12, 10);
    assert(buffer != NULL);

    string_holder = GapBufferGetString(buffer3);
    string_comp_assert(string_holder, sample12);

    printf("Test 6.1 Create From string\n");

    err = GapBufferInsertChar(buffer3, 'p');
    assert(err == 0);

    string_holder = GapBufferGetString(buffer3);
    string_comp_assert(string_holder, sample13);

    printf("Test 7 Backspace\n");
    for (int i=0; i<7; i++){
        GapBufferBackSpace(buffer);
    }

    string_holder = GapBufferGetString(buffer);
    string_comp_assert(string_holder, sample1);


    printf("Cleanup...\n");
    DestroyGapBuffer(buffer);
    DestroyGapBuffer(buffer2);
    DestroyGapBuffer(buffer3);

    printf("GapBuffer Tests Passed.");
}





void TestTextBuffer(){

    printf("\n\nTesting TextBuffer\n");

    TextBuffer* texBuffer = CreateTextBuffer(10, 20);
    assert(texBuffer != NULL);

    int errno;
    char* string_holder = NULL;

    const char sample1[] = "";
    const char sample2[] = "a";
    const char sample3[] = "aaaaaaaaaaa";  // 11 characters
    const char sample4[] = "aaaaaaaaaa";  // 10 characters
    const char sample5[] = "aaaaaaaaa";  // 9 characters


    printf("Test 1, empty string\n");

    string_holder = TextBufferGetLine(texBuffer, texBuffer->cursorRow);
    string_comp_assert(string_holder, sample1);

    printf("Test 2 insert char\n");
    errno = TextBufferInsert(texBuffer, 'a');
    assert(errno == 0);

    string_holder = TextBufferGetLine(texBuffer, texBuffer->cursorRow);
    string_comp_assert(string_holder, sample2);

    printf("Test 2.1 insert char\n");
    for (int i=0; i<10; i++){
        errno = TextBufferInsert(texBuffer, 'a');
        assert(errno == 0);
    }
    string_holder = TextBufferGetLine(texBuffer, texBuffer->cursorRow);
    string_comp_assert(string_holder, sample3);

    printf("Test 3 Backspace \n");
    TextBufferBackspace(texBuffer);
    string_holder = TextBufferGetLine(texBuffer, texBuffer->cursorRow);
    string_comp_assert(string_holder, sample4);

    printf("Test 4 MoveCursor, Newline\n");
    TextBufferMoveCursor(texBuffer, texBuffer->cursorRow, texBuffer->cursorCol-1);

    errno = TextBufferNewLine(texBuffer);
    assert(errno == 0);

    string_holder = TextBufferGetLine(texBuffer, texBuffer->cursorRow);
    string_comp_assert(string_holder, sample2);

    string_holder = TextBufferGetLine(texBuffer, texBuffer->cursorRow-1);
    string_comp_assert(string_holder, sample5);


    printf("Test 5 Create from file\n");
    TextBuffer* textBuffer2 = CreateTextBufferFromFile(test_fp);
    assert(textBuffer2 != NULL);
    assert(textBuffer2->cursorRow == 0);
    assert(textBuffer2->cursorCol == 0);
    assert(textBuffer2->last_line_loc == 2);

    string_holder = TextBufferGetLine(textBuffer2, 0);
    string_comp_assert(string_holder, sample3);

    string_holder = TextBufferGetLine(textBuffer2, 1);
    string_comp_assert(string_holder, sample4);

    string_holder = TextBufferGetLine(textBuffer2, 2);
    string_comp_assert(string_holder, sample5);

    printf("Cleanup...\n");
    DestroyTextBuffer(texBuffer);


    printf("TextBuffer Tests Passed.\n");
}