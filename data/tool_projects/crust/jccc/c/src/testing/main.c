#include "codegen/x86/test_x86.h"
#include "lexer/test_lexer.h"
#include <util/test_list.h>
#include <util/test_util.h>

int main() {
	test_lexer();
	test_x86();
	test_list();
	test_util();

	return 0;
}
