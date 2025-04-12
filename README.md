# KyrylScript v1.0

KyrylScript is a statically-typed scripting language built in Rust. It features first-class functions, a minimal standard library, and a simple yet expressive syntax. Designed with clarity, type safety, and extensibility in mind, it is suitable both for language design learning and lightweight scripting tasks.

---

## 1. Language Introduction

KyrylScript is a scripting language implemented from scratch. It aims to provide a clean syntax, strict typing, and dynamic evaluation — all driven by an interpreter written in Rust. The project demonstrates how a real language can be designed, parsed, interpreted, and extended.

---

## 2. Specification

### 2.1 Syntax

#### 2.1.1 Language Tokens

KyrylScript uses the following token types:

- **Literals**: `Integer`, `Float`, `String`, `Boolean`
- **Keywords**: `function`, `return`, `let`, `if`, `else`, `while`, `true`, `false`
- **Operators**: `+`, `-`, `*`, `/`, `==`, `!=`, `<`, `>`, `<=`, `>=`, `&&`, `||`, `~`, `^`
- **Delimiters**: `(`, `)`, `{`, `}`, `[`, `]`, `:`, `;`, `,`

#### 2.1.2 Syntax in BNF Format

```bnf
<program> ::= { <function> }

<function> ::= "function" <identifier> "(" [<parameter> {"," <parameter>}] ")" [":" <type>] "{" <statements> "}"

<parameter> ::= <identifier> ":" <type>

<statements> ::= { <statement> }

<statement> ::= "let" <identifier> ":" <type> "=" <expression> ";"
              | <identifier> "=" <expression> ";"
              | "return" <expression>? ";"
              | <expression> ";"
              | "if" <expression> "{" <statements> "}" ["else" "{" <statements> "}"]
              | "while" <expression> "{" <statements> "}"

<expression> ::= <literal>
               | <identifier>
               | <expression> <binary_op> <expression>
               | <unary_op> <expression>
               | <identifier> "(" [<expression> {"," <expression>}] ")"
               | "(" <expression> ")"

<binary_op> ::= "+" | "-" | "*" | "/" | "==" | "!=" | "<" | ">" | "<=" | ">=" | "&&" | "||"

<unary_op> ::= "-" | "~"

<type> ::= "int" | "float" | "string" | "bool" | "void"
```
