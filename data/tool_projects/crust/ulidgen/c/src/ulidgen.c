/*
 * ulidgen - generate or tag lines with ULID
 * (Universally Unique Lexicographically Sortable Identifier)
 *
 * Usage: ulidgen [-n N | -t]
 *    -n N   generate N ULID (default: 1)
 *    -t     print each line of standard input prefixed with an ULID
 *
 * To the extent possible under law, Leah Neukirchen <leah@vuxu.org>
 * has waived all copyright and related or neighboring rights to this work.
 * http://creativecommons.org/publicdomain/zero/1.0/
 */

#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

#include "ulid.h"

int
main(int argc, char *argv[])
{
	char ulid[27] = { 0 };

	int c;
	long n = 1;
	int tflag = 0;
        while ((c = getopt(argc, argv, "n:t")) != -1)
	        switch (c) {
	        case 'n': n = atol(optarg); break;
	        case 't': tflag = 1; break;
	        }

	if (tflag) {
		char *line = 0;
		size_t linelen = 0;

		setvbuf(stdout, 0, _IOLBF, 0);

		while (getdelim(&line, &linelen, '\n', stdin) != -1) {
			ulidgen_r(ulid);
			printf("%s %s", ulid, line);
		}
	} else {
		for (long i = 0; i < n; i++) {
			ulidgen_r(ulid);
			puts(ulid);
		}
	}

	fflush(0);
	exit(!!ferror(stdout));
}
