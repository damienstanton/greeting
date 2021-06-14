default: demo

demo:
	@cargo b --release && cd client && ./build hello.cpp && ./hello
