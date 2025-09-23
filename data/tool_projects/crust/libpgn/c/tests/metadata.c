#include "extra/suite6.h"

#include <pgn.h>
#include <string.h>

void test_parsing_metadata()
{
    pgn_metadata_t *metadata = pgn_parse_metadata(
        "[Event \"F/S Return Match\"]\n"
        "[Site \"Belgrade, Serbia JUG\"]\n"
        "[Date \"1992.11.04\"]\n"
        "[Round \"29\"]\n"
        "[White \"Fischer, Robert J.\"]\n"
        "[Black \"Spassky, Boris V.\"]\n"
        "[Result \"1/2-1/2\"]\n");

    suite6_assert(strcmp(pgn_metadata_get(metadata, "Event"), "F/S Return Match") == 0);
    suite6_assert(strcmp(pgn_metadata_get(metadata, "Site"), "Belgrade, Serbia JUG") == 0);
    suite6_assert(strcmp(pgn_metadata_get(metadata, "Date"), "1992.11.04") == 0);
    suite6_assert(strcmp(pgn_metadata_get(metadata, "Round"), "29") == 0);
    suite6_assert(strcmp(pgn_metadata_get(metadata, "White"), "Fischer, Robert J.") == 0);
    suite6_assert(strcmp(pgn_metadata_get(metadata, "Black"), "Spassky, Boris V.") == 0);
    suite6_assert(strcmp(pgn_metadata_get(metadata, "Result"), "1/2-1/2") == 0);
    suite6_assert(pgn_parse_score(pgn_metadata_get(metadata, "Result")) == PGN_SCORE_DRAW);
    pgn_metadata_cleanup(metadata);
}

int main(void)
{
    test_parsing_metadata();
    return 0;
}
