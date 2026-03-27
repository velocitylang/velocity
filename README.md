# Velocity

The vision for this AI-first language is to be fast and deterministic, with first-class citizens such as:

- tensors
- shapes
- gradients
- kernels

## Intro

Velocity files have a `.vl` extension.

We're in extremely early development, and the Velocity language is not usable. You're welcome to clone the repo, tweak the `main.vl` file, and execute `cargo run` to see some internals.

## Identifiers

You can assign numbers, strings, and booleans to identifiers.

- `let`: define an immutable value. e.g. `let base = 10`
- `mut` modifier: define a mutable value. e.g. `let mut x = 0`

## Math

You can add, subtract, multiply, and divide numbers.

## Printing to the console

You can print values or expressions.

- `print("Hello Velocity!")`
- `print(x + 3)`

## Types

Supported types are:
- i8, i16, i32, i64
- u8, u16, u32, u64
- f32, f64
- string
- bool

```vl
let x: i8 = 3
```

Type declaration is not required; we infer the type when needed. The default numeric type is `i64`

## Control Flow

### If
`if` is an expression, so it can evaluate to a value or just be used for side effects.

```vl
let a = if true { 100 } else { 50 }

if true { print("is true") } else { print("is false") }
```
