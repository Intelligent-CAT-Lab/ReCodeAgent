#ifndef SIMPLE_LANG_TOKEN_H
#define SIMPLE_LANG_TOKEN_H

typedef enum {
    TOKEN_INT,
    TOKEN_IDENTIFIER,
    TOKEN_ASSIGN,
    TOKEN_PLUS,
    TOKEN_MINUS,
    TOKEN_SEMICOLON,
    TOKEN_LET,
    TOKEN_EOF,
    TOKEN_DIS   //display
} TokenType;

typedef struct {
    TokenType type;
    char* value;
} Token;

Token * new_token(TokenType type, const char* value);
void free_token(Token* token);

#endif //SIMPLE_LANG_TOKEN_H
