/* This testing case is make sure ljmm works in child process */

#include <sys/mman.h>
#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>

#define ADDR_1G  0x40000000
#define ADDR_2G  0x80000000

static int
mytest() {
    char *p = malloc(12);
    if (p <= (char*)sbrk(0)) {
        fprintf(stderr, "malloc() still allocate from heap area\n");
        return 0;
    }

    p = mmap(NULL, 12, PROT_READ|PROT_WRITE,
             MAP_32BIT|MAP_PRIVATE|MAP_ANONYMOUS, -1, 0);

    if (p >= (char*)ADDR_1G || p <= (char*)sbrk(0)) {
        fprintf(stderr, "mmap wrapper dose not work\n");
        return 0;
    }

    return 1;
}

int
main(int argc, char **argv) {
    ljmm_let_OS_take_care_1G_2G(0);

    pid_t pid = fork();
    if (pid < 0) {
        return 1;
    }

    if (pid) {
        /* parent process */
        int child_status;
        waitpid(pid, &child_status, 0);
        if (!WIFEXITED(child_status) || WEXITSTATUS(child_status) != 0) {
            fprintf(stderr, "child process does not exit normally\n");
            exit(1);
        }
    }

    exit(mytest() ? 0 : 1);
}
