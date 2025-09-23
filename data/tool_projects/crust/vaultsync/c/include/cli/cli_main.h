#ifndef CLI_MAIN_H
#define CLI_MAIN_H

#define ARG_INIT "init"
#define ARG_HELP "--help"
#define ARG_HELP_SC "-h"
#define ARG_ADD_CHANGES "add"
#define ARG_COMMIT "commit"
#define ARG_ROLLBACK "rollback"

void run_cli(int argc, char const *argv[]);
void getting_help();
void invalid_args();

#endif