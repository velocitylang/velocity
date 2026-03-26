# Velocity

A deterministic, AI-first programming language.

I have no idea how far this is going to go. I'm writing this on March 8th, 2026. I started building this language, solely with AI, a week ago. There were over 33k lines of code, excluding the test suite, and I had no idea what the hell was going on. I burned it; burned it all.

It would be awesome to build a new language that is intended for artificial intelligence and machine learning, and hopefully adaptive enough for whatever comes next. It should guarantee determinism, provide great syntax, and be fast.

If you're interested in coming along for the ride, and you know Rust and/or the innerworkings of AI and ML, hit me up on X @j4w8n or Bluesky @j4w8n.com

Right now, I'm off to see if I can get a simple lexer, AST, and compiler working - with only minor help from AI.

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

Type declaration is not required; we infer the type when needed. Default numeric type is `i64`

## Control Flow

### If
`if` is an expression, so it can evaluate to a value or just be used for side effects.

```vl
let a = if true { 100 } else { 50 }

if true { print("is true") } else { print("is false") }
```
