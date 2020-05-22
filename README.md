# kaitai-rs

A reimplementation of the Kaitai struct compiler in Rust

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
