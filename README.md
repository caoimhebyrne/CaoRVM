# Caoimhe's Rust JVM (CaoRVM)

This is a just-for-fun™ JVM implementation in Rust. This project was inspired by one of my previous C++ projects, [CaoVM](https://github.com/caoimhebyrne/caovm).

## Status

I am currently working on **Class File Parsing**.

## Why?

I've been coding in Java for almost 10 years now (_wow_), it was the first language I learned after I was introduced to coding when I was 7 years old. I was always fascinated by how it worked, but never looked too deep into it until I explored the world of [SpongePowered/Mixin](https://github.com/SpongePowered/Mixin), [ASM](https://asm.ow2.io/), and other runtime class modification utilities.

These utilities made me want to know more about Java and the JVM, so I made my own mediocre runtime modification library - [Injector](https://github.com/caoimhebyrne/Injector). This satisfied my quest for knowledge at the time, but I eventually just wanted to implement my own JVM. This sparked me to make [CaoVM](https://github.com/caoimhebyrne/caovm) :^)

## What was wrong with CaoVM?

Whilst I didn't get far with the project, I felt like I was being restrained by some of the shortcomings of C++:

- It was a lot of effort to implement something, the [Lagom](https://github.com/SerenityOS/serenity/tree/master/Meta/Lagom) collection of libraries helped a lot, but everything still felt like it could be done better.
- Rust enums are superior to C++'s enum system, or even Java's enum system. When I'm implementing the actual virtual machine, I feel like the enum system will help a lot when dealing with values, etc.
- I want to learn more Rust! I love C++, and I will still use it for my operating systems projects, and of course contributing to SerenityOS. But, it just didn't feel right for this project. Rust feels like a much better candidate, and I feel like there's so much that I still have to learn about Rust.
  - For example, the [`HexValue` struct](src/values/hex.rs) helped me to uncover [the Extension Trait pattern](https://rust-lang.github.io/rfcs/0445-extension-trait-conventions.html), something which I don't think is possible to fully replicate in C++ (without some weird hax, that is).

## Goals

- To create a JVM implementation which is able to run a [hello world program](./tests/Main.java).
  - This doesn't have to neccessarily be spec-compliant, however we should try to implement of the fundemental details in the Java Virtual Machine spec to try to be as compatible as possible.
- To have fun! This project isn't serious, it's just something I've been interested in for so long.

## License

This project is licensed under the [MIT](https://choosealicense.com/licenses/mit/) license.
