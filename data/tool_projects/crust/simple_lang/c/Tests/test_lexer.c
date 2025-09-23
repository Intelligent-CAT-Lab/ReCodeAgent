#include "lexer.h"
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

void test_tokenize() {
    const char* source = "let x = 5 + 3 - 2; dis x;";
    Token* tokens = tokenize(source);

    assert(tokens[0].type == TOKEN_LET);
    assert(tokens[1].type == TOKEN_IDENTIFIER);
    assert(tokens[2].type == TOKEN_ASSIGN);
    assert(tokens[3].type == TOKEN_INT);
    assert(tokens[4].type == TOKEN_PLUS);
    assert(tokens[5].type == TOKEN_INT);
    assert(tokens[6].type == TOKEN_MINUS);
    assert(tokens[7].type == TOKEN_INT);
    assert(tokens[8].type == TOKEN_SEMICOLON);
    assert(tokens[9].type == TOKEN_DIS);
    assert(tokens[10].type == TOKEN_IDENTIFIER);
    assert(tokens[11].type == TOKEN_SEMICOLON);
    assert(tokens[12].type == TOKEN_EOF);

    int i = 0;
//    for (; tokens[i].type != TOKEN_EOF; i++) {
//        //fprintf(stderr,"releasing ->  %d", i);
//        free_token(&tokens[i]);
//    }
//    // 释放EOF标记
//    //free_token(&tokens[i]);
    free(tokens);
}

int main() {
    test_tokenize();
    printf("All Tests passed!\n");
    return 0;
}
