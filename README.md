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
- [] (array)

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

## Arrays

Arrays must have all items be of the same type.

Strict immutability is enforced for arrays. Meaning that if you want to add, change, or remove items, you need to declare with `mut`.

### Dynamic

```vl
let sizes: i64[] = [0, 7, 12]

// Types are inferred if omitted
// Here, we're inferring an array of `string`s
let groups = ["beginner", "intermediate", "advanced"]
```

### Fixed

To create a fixed-size array, declare the number of items it will hold, surrounded by brackets, with the type declaration.

```vl
let things: string[3] = ["hat", "gloves", "coat"]

// As with dynamic arrays, type is inferred if omitted
// Here, we're declaring it's an array of 4 items, and we infer `i64` for the items
let stuff: [4] = [1, 2, 3, 4]
```

## Tuples

Tuples can have mixed-type collections. Note that you don't define a type for tuple items, since they can be of mixed types.

Strict immutability is enforced for tuples. Meaning that if you want to add, change, or remove items, you need to declare with `mut`.

### Dynamic

```vl
let data = (1, "three", false)
```

### Fixed

To create a fixed-size tuple, declare the number of items it will hold, surrounded by parenthesis.

```vl
let things: (3) = ["hat", "gloves", "coat"]
```
