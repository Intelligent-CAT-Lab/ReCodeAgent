#include <stdio.h>

#include "CuTest.h"

CuSuite* CardUtilSuite();
CuSuite* HandUtilSuite();
CuSuite* BucketSuite();
CuSuite* DeckSuite();

void RunAllTests(void) {
    CuString* output = CuStringNew();
    CuSuite* suite = CuSuiteNew();

    CuSuiteAddSuite(suite, CardUtilSuite());
    CuSuiteAddSuite(suite, HandUtilSuite());
    CuSuiteAddSuite(suite, BucketSuite());
    CuSuiteAddSuite(suite, DeckSuite());

    CuSuiteRun(suite);
    CuSuiteSummary(suite, output);
    CuSuiteDetails(suite, output);
    printf("%s\n", output->buffer);
}

int main(void) {
    RunAllTests();

    return 0;
}
