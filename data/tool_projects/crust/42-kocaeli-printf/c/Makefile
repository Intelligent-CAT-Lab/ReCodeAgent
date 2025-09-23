NAME = libftprintf.a

SRC = src/ft_printf.c 
OBJ = $(SRC:.c=.o)

CC = gcc
CFLAGS = -Wall -Wextra -fprofile-arcs -ftest-coverage -fPIC

LDFLAGS = -lgcov --coverage

all: $(NAME)

$(NAME): $(OBJ)
	@ar rcs $(NAME) $(OBJ)

$(OBJ): src/ft_printf.h src/ft_printf.c

%.o: %.c src/ft_printf.h
	@$(CC) $(CFLAGS) -c $< -o $@

clean:
	@rm -rf $(OBJ) *.gcno *.gcda *.gcov
	@rm -rf **/*.gcno **/*.gcda **/*.gcov

fclean: clean
	@rm -rf $(NAME) test_1

re: fclean all

.PHONY: all clean fclean re test

# ✅ Fixed test rule with coverage flags
test: $(NAME)
	@$(CC) $(CFLAGS) tests/writenumber_test.c $(NAME) -o test_1 $(LDFLAGS)
	@./test_1

# ✅ Coverage report generation
coverage: test
	@gcov -b -c $(SRC)
