#include "settings.h"
#include "lexer.h"
#include "utils.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>



Token *
new_token(TokenType type, const char* value)
{
    Token * token = (Token*) malloc(sizeof(Token));
    token -> type = type;
    token -> value = strdup(value);
    return token;
}

void
free_token(Token* token)
{
    free(token -> value);
    free(token);
}

Token*
tokenize(const char* source)
{
    Token * tokens = (Token *) malloc(sizeof(Token) * MAX_TOKEN);
    int pos = 0;
    int len = strlen(source);
    int index = 0;

    while (pos < len) {
        if (isspace(source[pos])) {
            pos++;
        }
        else if (isdigit(source[pos])) {
            int start = pos;
            while (isdigit(source[pos])) { pos++; }
            char* num = strndup(&source[start], pos-start);
            tokens[index] = *new_token(TOKEN_INT, num);
            index++;
            free(num);
        }
        else if (isalpha(source[pos])) {
            int start = pos;
            while (isalpha(source[pos])) { pos++; }
            char* indent = strndup(&source[start], pos-start);
            if (strcmp(indent, "let") == 0) {
                tokens[index] = *new_token(TOKEN_LET, indent);
                index++;
            }
            else if (strcmp(indent, "dis") == 0) {
                tokens[index] = *new_token(TOKEN_DIS, indent);
                index++;
            }
            else {
                tokens[index] = *new_token(TOKEN_IDENTIFIER, indent);
                index++;
            }
            free(indent);
        }
        else {
            switch (source[pos]) {
                case '+':
                    tokens[index] = *new_token(TOKEN_PLUS, "+");
                    index++;
                    break;
                case '-':
                    tokens[index] = *new_token(TOKEN_MINUS, "-");
                    index++;
                    break;
                case ';':
                    tokens[index] = *new_token(TOKEN_SEMICOLON, ";");
                    index++;
                    break;
                case '=':
                    tokens[index] = *new_token(TOKEN_ASSIGN, "=");
                    index++;
                    break;
                default:
                    fprintf(stderr, "SyntaxError: unexpected character %c\n", source[pos]);
                    exit(1);
            }
            pos++;
        }
    }

    tokens[index] = *new_token(TOKEN_EOF, "");
    return tokens;
}
