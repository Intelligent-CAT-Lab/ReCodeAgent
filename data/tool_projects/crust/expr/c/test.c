
#include <math.h>
#include <stdio.h>

#include "mapper_expr.h"

int test1()
{
    const char str[] = "y=26*2/2+log10(pi)+2.*pow(2,1*(3+7*.1)*1.1+x{-6*2+12})*3*4+cos(2.)";
    mapper_expr e = mapper_expr_new_from_string(str, 1, 1, 1);
    printf("Parsing %s\n", str);
    if (!e) { printf("Test FAILED.\n"); return 1; }
#ifdef DEBUG
    printexpr("Parser returned: ", e);
#endif
    printf("\n");

    float inp, outp;
    inp = 3.0;
    mapper_expr_evaluate(e, &inp, &outp);

    printf("Evaluate with x=%f: %f (expected: %f)\n",
           inp, outp,
           26*2/2+log10f(M_PI)+2.f*powf(2,1*(3+7*.1f)*1.1f+inp)*3*4+cosf(2.0f));

    mapper_expr_free(e);

    return 0;
}

int test2()
{
    const char str[] = "y=26*2/2+x*30/(20*1)";
    mapper_expr e = mapper_expr_new_from_string(str, 0, 0, 1);
    printf("\nParsing %s\n", str);
    if (!e) { printf("Test FAILED.\n"); return 1; }
#ifdef DEBUG
    printexpr("Parser returned: ", e);
#endif
    printf("\n");

    int inp, outp;
    inp = 3;
    mapper_expr_evaluate(e, &inp, &outp);

    printf("Evaluate with x=%d: %d (expected: %d)\n",
           inp, outp,
           26*2/2+inp*30/(20*1));

    inp = 321;
    mapper_expr_evaluate(e, &inp, &outp);

    printf("Evaluate with x=%d: %d (expected: %d)\n",
           inp, outp,
           26*2/2+inp*30/(20*1));

    mapper_expr_free(e);

    return 0;
}

int main()
{
    int rc=0;
    rc|=test1();
    rc|=test2();
    return rc;
}
