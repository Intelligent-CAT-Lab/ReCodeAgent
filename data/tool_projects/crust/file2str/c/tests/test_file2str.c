#include <stdbool.h>
#include <assert.h>
#include <setjmp.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include "CuTest.h"

#include "file2str.h"

void TestFile2str_file_read_properly(CuTest * tc)
{
    unsigned int len;

    char* buf = file2strl("tests/test.txt", &len);

    CuAssertTrue(tc, 11 == len);
    CuAssertTrue(tc, 0 == strcmp(buf, "Text file\n"));
    free(buf);
}
