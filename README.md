# RUST

- Command to not make a git repo inside a Rust initial folder  
  `cargo new my_sqlite_project --vcs none`

- Before running a RUST program, you must compile it with `rustc main.rs`.  
  It will give a binary executable `main.exe`. This is good for small codes and files.

- `cargo build` creates the executable in `rust_hello/target/debug/rust_hello.exe`

- You can also run it via `cargo run`. For complex and large files use this.  
  It also doesn't need `cargo build` to be run before.

- Cargo is Rust’s build system and package manager.

- In Rust, packages of code are referred to as crates.

- Cargo also provides a command called `cargo check`.  
  This command quickly checks your code to make sure it compiles but doesn’t produce an executable.

- When your project is finally ready for release, you can use `cargo build --release`  
  to compile it with optimizations. This command will create an executable in `target/release`  
  instead of `target/debug`.

- To work on any existing projects:
  ```
  git clone example.org/someproject
  cd someproject
  cargo build
  ```

---

# Chapter 2

- By default, Rust has a set of items defined in the standard library that it brings into the scope of every program. This set is called the prelude.

- In Rust, variables are immutable by default, meaning once we give the variable a value, the value won’t change.

- A crate is a collection of Rust source code files.

---

# Chapter 3

- In languages with a garbage collector (GC), the GC keeps track of and cleans up memory that isn’t being used anymore, and we don’t need to think about it.

- In most languages without a GC, it’s our responsibility to identify when memory is no longer being used and to call code to explicitly free it.

- Problems:
  - If we forget → memory waste
  - If we do it too early → invalid variable
  - If we do it twice → bug
  - We need exactly one allocate with one free

---

- When we assign `s1` to `s2`, the String data is not fully copied.

- We copy:
  - pointer
  - length
  - capacity

- We do NOT copy the data on the heap.

---

- Types such as integers have a known size at compile time and are stored entirely on the stack, so copies are quick.

---

- Creating a reference `&s` is called Borrowing.

---

- Mutable references rule:
  - If you have a mutable reference, you can have no other references.
  - You also cannot have a mutable reference while having immutable ones.

```
let mut s = String::from("hello");
let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM

println!("{}, {}, and {}", r1, r2, r3);
```

---

- Dangling pointer:
  - A pointer that references a memory location that may have been freed or given to someone else.

- In Rust:
  - Compiler guarantees references will never be dangling
  - Data will not go out of scope before its reference

---

## The Rules of References

- At any given time:
  - Either one mutable reference  
  - OR any number of immutable references

- References must always be valid

-  Internally, the slice data structure stores the starting position and the length of the slice, which corresponds to ending_index minus starting_index . So, in the case of let world = &s[6..11]; , world would be a slice that contains a pointer to the byte at index 6 of s with a length value of 5 .