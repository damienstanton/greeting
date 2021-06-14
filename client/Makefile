default: c

c:
	@echo "Compiling ${f}..."
	@clang++ -Wall -pipe -O2 -std=c++17 \
	-L ../target/release \
	-l helloc \
	-Wall -Wextra -pedantic -Wshadow -Wformat=2 -Wfloat-equal -Wconversion \
	-Wshift-overflow -Wcast-qual -Wcast-align \
	-Wno-unused-result -Wno-sign-conversion \
	${f} -o $$(basename -s .cpp ${f}) \
	&& echo "Compiled without errors"
