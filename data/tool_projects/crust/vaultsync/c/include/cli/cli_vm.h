#ifndef CLI_VM_H
#define CLI_VM_H

void init_repository();
void track_changes(int n, char const *files[]);
void commit_changes();
void rollback_changes(const char* hash);

#endif