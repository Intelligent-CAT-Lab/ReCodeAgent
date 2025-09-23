/**
 * Main C source code file for the JabaCat C Compiler.
 */

#include <stdio.h>
#include <string.h> // strcmp

#include <lexer/lex.h>
#include <parser/parse.h>
#include <util/out.h>

int lexer_dump(const char *filename) {

    // Initialization of everything
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
    do {
        // Return if some non-zero (error) code is returned
        if (lex(&lexer, &t))
            return 1;
        printf("Contents: %20s, type: %20s, position: %d/%d\n", t.contents,
               ttype_name(t.type), t.line, t.column);
    } while (t.type != TT_EOF);

	fclose(fp);

    return 0;
}

int main(int argc, char **argv) {

    // TODO -- move this out of main function, perhaps?

    // Skip the name of the executable.
    --argc, ++argv;

    if (argc == 0) {
        PRINT_DEFAULT("Usage: --token-dump <filename> to see all tokens");
        return 0;
    }

    if (argc == 1) {
        PRINT_DEFAULT("default compilation not supported yet -- try 'jccc "
                      "--token-dump %s' instead.",
                      argv[0]);
        return 1;
    }

    if (argc > 2) {
        PRINT_DEFAULT("expected only two arguments!");
        return 1;
    }

    // Two arguments now.
    if (strcmp(argv[0], "--token-dump") == 0) {
        // Finally, we can do the lexer test properly!
        return lexer_dump(argv[1]);
    } else if (strcmp(argv[0], "--test-parse") == 0) {
		parse(argv[1]);
		return 0;
	}

	PRINT_ERROR("option %s not recognized.", argv[1]);
	return 1;
}
