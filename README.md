# KyrylScript

**KyrylScript** is a lightweight interpreted programming language designed and built from scratch in Rust.  
It combines the simplicity of scripting with the clarity of structured typing, offering a unique reference-based runtime without garbage collection.

> A spiritual successor to Lua, with the structural rigor of Rust and the flexibility of Python.

---

## Features

- **Custom Lexer and Parser**
  - Hand-written in Rust, with clear syntax trees and tokenization rules.

- **Runtime and Scoping**
  - Lexical scoping with reference-based variable tracking.
  - No garbage collector — all memory is managed manually using reference IDs.

- **Type System**
  - Built-in support for numbers, strings, booleans, lists, tuples, functions, and native values.

- **Control Structures**
  - If/else, while loops, pattern matching, recursion, scoped blocks.

- **Native Function Registry**
  - Easily extend the language by registering native Rust functions with the runtime.

---

## Getting Started

1. **Clone the repo**

```bash
git clone https://github.com/yourname/kyrylscript.git
cd kyrylscript
```

2. **Build & Run**

```bash
cargo run -- examples/test.ks
```

---

## Roadmap

* [x] Lexing, Parsing, Runtime
* [x] Custom Value Model
* [x] Native Function Integration
* [x] Basic Control Structures
* [x] Functional Programming Support
* [ ] Generics
* [ ] Module System & Imports
* [ ] REPL
* [ ] Object-Oriented Programming System
* [ ] Online Playground

---

## License

MIT © 2025 Kyryl Lebedenko

---

> Created with love by Kyryl Lebedenko 

