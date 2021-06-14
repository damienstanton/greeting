#include <iostream>

extern "C" {
	char* greeting_c(const char *);
}

int main() {
	auto name = "Damien";
	std::cout << greeting_c(name) << std::endl;
}
