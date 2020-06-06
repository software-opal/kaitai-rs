# kaitai-rs

A reimplementation of the Kaitai struct compiler in Rust

## This project is abandoned

After writing most of a parser I realised that it did not serve my needs. I will still use [Kaitai](https://kaitai.io/) for documentation in [my Machine Embroidery project](https://github.com/software-opal/embroidery-rust), however I have a particular vision for that library and the way it handles parsing files that this library would not be able to meet.

That and the project is not sparking joy any more.

---

## Why?

Because my brain decided that I wanted to sink immeasurable hours into making an inferior product rather than learn how to
read/write/compile/work with Scala. Also I want a pure rust build system for my Embroidery project. But mostly the first thing.

## Goals
I want this to:
- Pure rust code gen
- Be able to use it to load embroidery formats
- Minimal runtime dependencies

I do not want it to:
- Support all possible Kaitai definitions
