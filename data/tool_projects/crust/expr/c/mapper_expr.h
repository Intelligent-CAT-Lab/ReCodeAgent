
#ifndef __EXPR_H__
#define __EXPR_H__

/**** Expression parser/evaluator ****/

typedef struct _mapper_expr *mapper_expr;

mapper_expr mapper_expr_new_from_string(const char *str,
                                        int input_is_float,
                                        int output_is_float,
                                        int vector_size);

#ifdef DEBUG
void printexpr(const char*, mapper_expr);
#endif

int mapper_expr_evaluate(mapper_expr expr,
                         void* input_vector,
                         void* output_vector);

void mapper_expr_free(mapper_expr expr);

#endif // __EXPR_H__
