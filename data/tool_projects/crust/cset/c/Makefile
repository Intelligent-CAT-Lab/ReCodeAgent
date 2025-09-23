.PHONY: test

test: tests/test.c
	@$(CC) $^ -o $@ -g -lm
	echo "Running test..."
	@./$@
	echo "Test completed."
