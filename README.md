Requirements:

- A recent Rust compiler (tested on 1.52)
- A recent clang compiler (tested on Apple clang 12.0.5)
- gnu `make`
- `bear` (only if you want to run my script exactly as it is, to generate a `compile_commands.json` on every successful C++ build)

From the top of the workspace, simply running
```console
$ make
```

will do the following things:
- Compile our Rust library`helloc` into a cdylib. On macOS this produces a `.dylib` object. On Linux this would be a `.so`, on Windows a `.dll`.
- Compile our C++ program, linking in the newly created `libhelloc`
- Run our example C++ program, showing that we successfully called into a Rust library from C++
