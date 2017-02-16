#include <stdio.h>

extern void rust_main();

int main() {
	rust_main();
	return 0;
}

void print_string(const char* string, size_t size) {
	fwrite(string, sizeof(char), size, stdout);
}
