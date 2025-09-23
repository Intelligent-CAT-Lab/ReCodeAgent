#include <pgn.h>
#include <ctype.h>
#include <string.h>
#include <stdlib.h>
#include <stdio.h>

#include "fmt.h"

char *read_file(char *filename)
{
    FILE *fp = fopen(filename, "r");
    if (!fp) {
        fprintf(stderr, "ERROR: file '%s' is not found\n", filename);
        exit(EXIT_FAILURE);
    }

    fseek(fp, 0, SEEK_END);
    int filesize = ftell(fp);
    fseek(fp, 0, SEEK_SET);

    char *buf = malloc((sizeof *buf) * (filesize + 1));
    fread(buf, sizeof *buf, filesize, fp);
    buf[filesize] = '\0';

    fclose(fp);
    return buf;
}

void run_subcommand_fmt(int argc, char **argv)
{
    if (argc <= 2) {
        fprintf(stderr, "ERROR: no pgn file provided.\n");
        exit(EXIT_FAILURE);
    }

    char *filename = argv[2];
    char *pgn_file_contents = read_file(filename);
    char *saved_ptr = pgn_file_contents;

    while (*pgn_file_contents) {
        pgn_t *pgn = pgn_init();
        size_t consumed = pgn_parse(pgn, pgn_file_contents);

        pgn_file_contents += consumed;
        while (isspace(*pgn_file_contents))
            pgn_file_contents++;

        fmt_print(pgn);

        /* still have unparsed pgn */
        if (*pgn_file_contents)
            printf("\n");

        pgn_cleanup(pgn);
    }

    free(saved_ptr);
}

int main(int argc, char **argv)
{
    if (argc <= 1) {
        fprintf(stderr, "ERROR: no subcommand provided.\n");
        exit(EXIT_FAILURE);
    }

    char *subcommand = argv[1];
    if (strcmp(subcommand, "fmt") == 0) {
        run_subcommand_fmt(argc, argv);
        return 0;
    } 

    fprintf(stderr, "ERROR: unknown subcommand '%s'.\n", subcommand);
    return 1;
}
