# Kyryl Script

KyrylScript is a small functional programming language designed and implemented by Kyryl Lebedenko as both a learning project and a scripting foundation for other projects.

It draws inspiration from Lua, Rust, and C#, with practical features meant for game development, web development and scriptable runtime environments.

---

## Features

* Functional scripting style
* Static typing
* Interpreted runtime
* Native support for Rust

## Build

```sh
# Clone the repository
git clone https://github.com/your-username/kyrylscript.git
cd kyrylscript

# Build the project
cargo build --release

# Run the interpreter
cargo run -- path/to/script.kys
```

## Integration with Rust

You can register native Rust functions into the KyrylScript runtime:

```rs
interpreter.register_rust_function("println", kys_println);
interpreter.register_rust_function("len", kys_len);
```

This allows full integration between game engine core logic and high-level script behavior.

## Future Plans
* Module system
* Better REPL/debugger tools
* Error messages with position tracking
* `null` support with `?` early-return operator
* Add structs and enums
* Structs with access control: `public`, `private`, `protected`

## License
MIT License. See `LICENSE` for details.

---

// Created by Kyryl Lebedenko