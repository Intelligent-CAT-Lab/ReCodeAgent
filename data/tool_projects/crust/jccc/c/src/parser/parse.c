/* Parser
 *
 */

#include <stdlib.h> // calloc
#include <string.h> // strcmp
#include <ctype.h> // isdigit

#include <codegen/x86/codegen.h>
#include <lexer/lex.h>
#include <lexer/token.h>
#include <parser/cst.h>
#include <util/out.h>

int parse(const char *filename) {

    Lexer lexer;

    FILE *fp = fopen(filename, "r");

    if (!fp) {
        PRINT_ERROR("File %s not found", filename);
        return 1;
    }

    lexer.fp = fp;
    lexer.unlexed_count = 0;
    lexer.column = lexer.line = 1;

    Token t;

    int i = 0;
    int buffer_size = 16;
    Token *tokens = calloc(buffer_size, sizeof(Token));

    do {
        if (lex(&lexer, &t)) {
            return 1;
        }

        if (buffer_size <= i) {
            buffer_size *= 2;
            tokens = calloc(buffer_size, sizeof(Token));
        }

        tokens[i] = t;

        printf("Contents: %20s, type: %20s, position: %d/%d\n", t.contents,
               ttype_name(t.type), t.line, t.column);

        i++;
    } while (t.type != TT_EOF);

    // Main function
    if (tokens[0].type == TT_INT && tokens[1].type == TT_IDENTIFIER &&
        (strcmp(tokens[1].contents, "main") == 0)) {

        // Correct empty function body
        if (tokens[2].type == TT_OPAREN && tokens[3].type == TT_CPAREN &&
            tokens[4].type == TT_OBRACE) {

            // Return value
            if (tokens[5].type == TT_RETURN && tokens[6].type == TT_LITERAL &&
                isdigit(tokens[6].contents[0]) && tokens[7].type == TT_SEMI) {

                // Correct matched closed brace
                if (tokens[8].type == TT_CBRACE) {
                    printf("\n");

                    // Generate preamble main code
                    char *code_start = start_main();

                    printf(code_start);

                    // Add custom return code
                    char *code_end =
                        end_main_custom_return(atoi(tokens[6].contents));

                    printf(code_end);

                } else {
                    PRINT_ERROR("Wrong closing brace.\n");
                }
            } else {
                PRINT_ERROR("Return value is wrong.\n");
            }
        } else {
            PRINT_ERROR("Wrong main function body.\n");
        }
    } else {
        PRINT_ERROR("Not correct main function.\n");
    }

    fclose(fp);

    return 0;
}

int parse_simple_main_func() {}

/**
 * Proper parsing code below -- producing a concrete syntax tree from a file.
 * Each of these functions will probably reference the others numerous times.
 */

int parse_expr(Lexer* l, Expression* ex) {
    // TODO (just a literal or a function call for now).
}

int parse_funccall(Lexer* l, Expression* ex) {
    // TODO
}

int parse_blockstmt(Lexer* l, BlockStatement* bs) {
    // TODO
}

int parse_funcdecl(Lexer* l, FunctionDeclaration* fd) {
    // TODO
}

// Parse function -- takes a lexer and produces a concrete syntax tree. Fill the
// struct which we have given with the data.
int make_cst(Lexer* l, ConcreteFileTree* tree) {
    // TODO
}
