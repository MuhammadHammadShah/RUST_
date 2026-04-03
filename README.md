# RUST_

- Command to not make a git repo inside a Rust initial folder.
`cargo new my_sqlite_project --vcs none`

- Before running a RUST program, you must compile it with `rustc main.rs`. it will give a binary executable `main.exe`. This is good for compiling small codes and files.
- `cargo build` create the executable in `rust_hello/target/debug/rust_hello.exe`
- you can also run it via `cargo run`. For complex and large files use this. It also doesn't need `cargo build` to be run before.
- Cargo is Rust’s build system and package manager.
- In Rust, packages of code are referred to as crates.
- Cargo also provides a command called `cargo check` . This command quickly checks your
code to make sure it compiles but doesn’t produce an executable.
- When your project is finally ready for release, you can use `cargo build --release` to
compile it with optimizations. This command will create an executable in `target/release`
instead of `target/debug`.
- To work on any existing projects, you can use the
following commands to check out the code using Git, change to that project’s directory, and
build:
```
$ git clone example.org/someproject
$ cd someproject
$ cargo build
```
------

# Chapter 2

- By default, Rust has a set of items defined in the standard library that it brings into the
scope of every program. This set is called the prelude.
- In Rust, variables are immutable by default, meaning once we give the variable a value, the value won’t change.
- A crate is a collection of Rust source code files. 

# chapter 3 

``` In languages with a garbage collector (GC), the GC
keeps track of and cleans up memory that isn’t being used anymore, and we don’t need to
think about it. In most languages without a GC, it’s our responsibility to identify when
memory is no longer being used and to call code to explicitly free it, just as we did to request
it. Doing this correctly has historically been a difficult programming problem. If we forget,
we’ll waste memory. If we do it too early, we’ll have an invalid variable. If we do it twice,
that’s a bug too. We need to pair exactly one allocate with exactly one free. ```




```When we assign s1 to s2 , the String data is copied, meaning we copy the pointer, the
length, and the capacity that are on the stack. We do not copy the data on the heap that the
pointer refers to.```

"""
The reason is that types such as integers that have a known size at compile time are stored
entirely on the stack, so copies of the actual values are quick to make.

"""

- Creating a reference `&s` is  called `Borrowing`