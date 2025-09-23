/**
 * The structures for a concrete syntax tree.
 * For now, the subset of C we are parsing is quite simple:
 * - Parameterless functions.
 * - Return statements, which accept integers or function calls.
 */

#pragma once

#include <util/list.h>

// A list of all node types.
typedef enum {
    NT_STMT,
    NT_EXPR,
    NT_BLOCK_STMT,
    NT_RETURN_STMT,
    NT_FUNCDECL,
    NT_FUNCCALL,
    NT_LITERAL,
} NodeType;

// A block statement is just a list of statements.
typedef struct {
    List* stmts; // A list of Statement structs.
} BlockStatement;

typedef struct {
    // TODO -- add parameters whe we get there
    BlockStatement body;
    char name[256]; // The actual name of the function.
} FunctionDeclaration;

// An entire program is just a list of top level declarations.
// For now, such declarations are only functions.
typedef struct {
    union {
        FunctionDeclaration fd;
        // VariableDeclaration vd; when we get there
    } u;
    NodeType type;
} TopLevelDeclaration;

// Right now, a function call doesn't have any parameters so it's just the name
// of the function being called.
typedef struct {
    char name[256];
} FunctionCall;

// An expression for now is an integer or a function call.
typedef struct {
    union {
        FunctionCall fc;
        char literal[256];
    } u;
    NodeType type;
} Expression;

// Finally, an entire source file is a list of top-level declarations.
typedef struct {
    List* decls; // list of TopLevelDeclaration
} ConcreteFileTree;
