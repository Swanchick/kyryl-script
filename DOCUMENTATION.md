# KyrylScript Documentation

KyrylScript is a statically-typed, high-level interpreted programming language designed to be simple, readable, and expressive. It provides essential control structures, basic data types, function support, and features like recursion, loops, and lists, making it suitable for both scripting and educational use.

---

## Table of Contents
- [Language Overview](#language-overview)
- [Syntax](#syntax)
- [Data Types](#data-types)
- [Control Flow](#control-flow)
- [Functions](#functions)
- [Lists and Indexing](#lists-and-indexing)
- [Loops](#loops)
- [Recursion](#recursion)
- [Comments](#comments)
- [Standard Library](#standard-library)
- [Examples](#examples)

---

## Language Overview
KyrylScript aims to offer:
- Clean, Python-inspired syntax
- Static and dynamic typing
- Support for nested data structures
- A straightforward function declaration system

---

## Syntax
### Variable Declaration
```kys
let a = 10;
let text: string = "Hello";
```

### Function Declaration
```kys
function add(a: int, b: int): int {
    return a + b;
}
```

---

## Data Types
- `int` – Integer numbers
- `float` – Floating-point numbers (`f` suffix)
- `string` – Text in double quotes
- `bool` – Boolean values (`true`, `false`)
- `[type]` – List of items of a given type

---

## Control Flow
### If-Else
```kys
if a == 10 {
    println("Yes");
} else {
    println("No");
}
```

### While Loop
```kys
while a < 10 {
    println(a);
    a += 1;
}
```

---

## Functions
Functions must be declared using `function`, can accept typed parameters, and return values of a specified type.

```kys
function multiply(x: int, y: int): int {
    return x * y;
}
```

Return type can be `void` if no return value:
```kys
function greet(name: string): void {
    println("Hello", name);
}
```

---

## Lists and Indexing
KyrylScript supports lists with type homogeneity:
```kys
let numbers: [int] = [1, 2, 3];
numbers[1] = 10;
```
Nested indexing is allowed:
```kys
let matrix = [[1, 2], [3, 4]];
println(matrix[1][0]);
```

---

## Loops
### For-Range Loop
```kys
for i in range(10) {
    println(i);
}
```

### For-In List Loop
```kys
for item in list {
    println(item);
}
```

---

## Recursion
KyrylScript supports recursive functions:
```kys
function fib(n: int): int {
    if n <= 1 {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}
```

---

## Comments
Use `//` to add single-line comments:
```kys
// This is a comment
```

---

## Standard Library
### Printing
- `print(...)` – Print without newline
- `println(...)` – Print with newline

### Utility
- `range(n)` – Returns a list from `0` to `n-1`
- `len(list)` – Returns length of list or string

---

## Examples
### Hello World
```kys
println("Hello, World!");
```

### Bubble Sort
```kys
function bubble_sort(numbers: [int]): [int] {
    for j in range(len(numbers)) {
        for i in range(len(numbers) - j - 1) {
            let n1 = numbers[i];
            let n2 = numbers[i + 1];

            if n1 > n2 {
                numbers[i] = n2;
                numbers[i + 1] = n1;
            }
        }
    }
    return numbers;
}
```

### Triangle Drawing
```kys
function draw_triangle(size: int) {
    while size > 0 {
        let row = 0;
        while row < size {
            print("*");
            row += 1;
        }
        println("");
        size -= 1;
    }
}
```

### Unit Converter
```kys
function to_meters(inches: float): float {
    return inches / 39.37f;
}
```

---

## Author
KyrylScript was designed and implemented by Kyryl Lebedenko Student of Vilnius Tech and Aalto University.

