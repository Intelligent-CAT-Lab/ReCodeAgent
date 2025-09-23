#include <stdlib.h>
#include "misc.h"


#define CASE_STR(x) case x: return #x; break;

char*
get_opcode_name(OpCode opcode) {
    switch (opcode){
        CASE_STR(LOAD_CONST)
        CASE_STR(LOAD_NAME)
        CASE_STR(STORE_NAME)
        CASE_STR(BINARY_SUB)
        CASE_STR(BINARY_ADD)
        CASE_STR(STK_DIS)
        default:
            return "UNKNOWN_OPCODE";
    }
}

char*
get_token_type_name(TokenType tokenType)
{
    switch (tokenType) {
        CASE_STR(TOKEN_INT)
        CASE_STR(TOKEN_IDENTIFIER)
        CASE_STR(TOKEN_ASSIGN)
        CASE_STR(TOKEN_PLUS)
        CASE_STR(TOKEN_MINUS)
        CASE_STR(TOKEN_SEMICOLON)
        CASE_STR(TOKEN_LET)
        CASE_STR(TOKEN_EOF)
        CASE_STR(TOKEN_DIS)
        default:
            return "UNKNOWN_TOKEN";
    }
}


void
show_opcode(Instruction * instruction, int instr_count)
{
    printf("bytecode:\n");
    for (int i = 0; i < instr_count; i++) {
        if (instruction[i].operand) {
            printf("%-15s \t (%s)\n",get_opcode_name(instruction[i].opcode), instruction[i].operand);
        }
        else {
            printf("%-15s \t *\n",get_opcode_name(instruction[i].opcode));
        }
    }
    printf("\n");
}

void
show_tokens(Token * tokens)
{
    printf("tokens:\n");
    for (int i = 0; tokens[i].type != TOKEN_EOF; i++) {
        printf("%-15s \t (%s)\n", get_token_type_name(tokens[i].type), tokens[i].value);
    }
    printf("\n");
}

void
print_ast(ASTNode* node, int level) {
    if (!node) return;

    // 打印缩进
    for (int i = 0; i < level; i++) {
        printf("  ");
    }

    // 打印节点信息
    switch (node->type) {
        case TOKEN_INT:
            printf("INT: %s\n", node->value);
            break;
        case TOKEN_IDENTIFIER:
            printf("IDENTIFIER: %s\n", node->value);
            break;
        case TOKEN_ASSIGN:
            printf("ASSIGN: %s\n", node->value);
            break;
        case TOKEN_LET:
            printf("LET: %s\n", node->value);
            break;
        case TOKEN_PLUS:
            printf("PLUS\n");
            break;
        case TOKEN_MINUS:
            printf("MINUS\n");
            break;
        case TOKEN_DIS:
            printf("DISPLAY\n");
            break;
        default:
            printf("UNKNOWN\n");
            break;
    }

    // 打印左子树和右子树
    if (node->left) {
        print_ast(node->left, level + 1);
    }
    if (node->right) {
        print_ast(node->right, level + 1);
    }
}

void
print_asts(ASTNode ** asts) {
    printf("AST:\n");
    for (int i = 0; asts[i] != NULL; i++) {
        print_ast(asts[i], 0);
        printf("\n");
    }
    printf("\n");
}

char *
read_file(const char * source_path)
{
    struct dirent *entry;
    char * source = (char *)malloc(sizeof(char) * MAX_SOURCE_LENGTH);

    FILE *file = fopen(source_path, "r");
    if (file == NULL) {
        perror("Could not open file");
        exit(-1);
    }

    int pos = 0;
    char ch;
    while ((ch = fgetc(file)) != EOF) {
        //putchar(ch);
        memcpy(&source[pos], &ch, sizeof(char));
        pos ++;
    }
    source[pos] = '\0';
    fclose(file);

    return source;
}