FLAGS := -g -Wall -Wextra -pedantic -std=c99 -fsanitize=address,leak,undefined
all: lib.c 01.c
	clang lib.c 01.c ${FLAGS}
