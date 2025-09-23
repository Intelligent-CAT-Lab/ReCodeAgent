# This Makefile is used to compile and link the test_assign3 program.

# Declare phony targets to avoid conflicts with files of the same name.
.PHONY: all clean

# The 'all' target is the default when 'make' is run without arguments.
all: test_assign3

# Compile and link the program 'test_assign3'.
test_assign3: storage_mgr.c test_assign3_1.c dberror.c buffer_mgr_stat.c buffer_mgr.c record_mgr.c rm_serializer.c expr.c
	
	gcc -o test_assign3 storage_mgr.c test_assign3_1.c dberror.c buffer_mgr_stat.c buffer_mgr.c record_mgr.c rm_serializer.c expr.c

	@echo ........Compilation and linking complete!!

# The 'clean' target is used to remove the compiled program.
clean:
	@echo "Cleaning up..."
	rm test_assign3
	@echo "Cleaned."

# End of the Makefile
