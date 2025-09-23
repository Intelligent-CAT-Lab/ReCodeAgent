#include<stdio.h>
#include "helpers/vector.h"
#include "compiler.h"
int main(){
    
    int res = compile_file("./test.txt","./test",0);
    if(res == COMPILER_FILE_COMPILED_OK)
    {
        printf("Compiled fine\n");
    }
    else if(res == COMPILER_FAILED_WITH_ERRORS)
    {
        printf("compile failed\n");
    }
    else
    {
        printf("Unknown response Compile failed\n");
    }

    return 0;
}