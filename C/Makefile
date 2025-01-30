CC=gcc
ECHO=echo -e

CFLAGS=-Wall -Werror -std=gnu99 -O0

FILES=build/variables1.o build/variables2.o build/variables3.o build/variables4.o build/variables5.o \
			build/variables_exercise1.o build/variables_exercise2.o build/variables_exercise3.o

all: $(FILES)
	@$(ECHO) "Build complete"

build/variables1.o: src/variables1.c
	@$(ECHO) "CC\t\t"$<
	@$(CC) $(CFLAGS) $< -o $@

build/variables2.o: src/variables2.c
	@$(ECHO) "CC\t\t"$<
	@$(CC) $(CFLAGS) $< -o $@

build/variables3.o: src/variables3.c
	@$(ECHO) "CC\t\t"$<
	@$(CC) $(CFLAGS) $< -o $@

build/variables4.o: src/variables4.c
	@$(ECHO) "CC\t\t"$<
	@$(CC) $(CFLAGS) $< -o $@

build/variables5.o: src/variables5.c
	@$(ECHO) "CC\t\t"$<
	@$(CC) $(CFLAGS) $< -o $@

build/variables_exercise1.o: src/variables_exercise1.c
	@$(ECHO) "CC\t\t"$<
	@$(CC) $(CFLAGS) $< -o $@

build/variables_exercise2.o: src/variables_exercise2.c
	@$(ECHO) "CC\t\t"$<
	@$(CC) $(CFLAGS) $< -o $@

build/variables_exercise3.o: src/variables_exercise3.c
	@$(ECHO) "CC\t\t"$<
	@$(CC) $(CFLAGS) $< -o $@

clean:
	@rm -f $(FILES)
