# Velocity

A deterministic, AI-first programming language.

I have no idea how far this is going to go. I'm writing this on March 8th, 2026. I started building this language, solely with AI, a week ago. There were over 33k lines of code, excluding the test suite, and I had no idea what the hell was going on. I burned it; burned it all.

It would be awesome to build a new language that is intended for artificial intelligence and machine learning, and hopefully adaptive enough for whatever comes next. It should guarantee determinism, provide great syntax, and be fast.

I enjoy doing things differently than others, so you're going to see that here. You've been warned.

If you're interested in coming along for the ride, and you know Rust and/or the innerworkings of AI and ML, hit me up on X @j4w8n or Bluesky @j4w8n.com

Right now, I'm off to see if I can get a simple lexer, AST, and compiler working - with only minor help from AI.

## Intro

Velocity files have a `.vl` extension. Since we're starting out with a VM-based system, the bytecode files will be `.vlb`.

## Identifiers

You can assign numbers, strings, and booleans to identifiers.

- `let`: define a mutable value. e.g. `let x = 0`
- `make`: define an immutable value. e.g. `make base = 10`

## Math

You can add, subtract, multiply, and divide numbers.

## CLI Tool

This will be called `velo`. Any compiler that we may build will be called `veloc`.
