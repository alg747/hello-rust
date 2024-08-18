-   [ ] install cargo
-   [x] hello world
-   [x] variable
    -   immutable by default
    -   `let`
    -   type
    -   `mut`
    -   constant
    -   shadowing
-   [x] scalar types
    -   i32, u32, f32, boolean, char
    -   integer overflow
    -   type conversion
-   [x] compound data types
    -   array
        -   array - collection of elements with length known at compile time
        -   slice - collection of elements with length known at runtime
    -   tuple
        -   pattern matching
-   [x] function
    -   no return value
    -   return value
    -   implicit return
-   [x] control flow
    -   [x] if / else
    -   [x] match
    -   [x] if let
    -   [x] let else
-   [x] loop
    -   [x] for and range
    -   [x] while
    -   [x] while let
    -   [x] returning values from loop
-   [x] returning values from block expression
-   [ ] ownership
    -   stack (last in, first out)
        -   store data with known fixed size
    -   heap
        -   vec (stack = vec pointer, data = heap)
    -   Each value in Rust has an owner.
    -   There can only be one owner at a time. (Transfer of ownership)
    -   When the owner goes out of scope, the value will be dropped.
    -   passing variable into function
-   [x] borrow
    -   reference
    -   mutable reference
    -   no dangling refe
-   [x] slice
-   [x] string and str
    -   str - string with length known at runtime
    - `r#`
-   [x] struct
    -   [x] update syntax
    -   [x] method (impl)
    -   [x] derive
-   [x] enum
-   [ ] error handling
    -   panic
    -   option
    -   result
    -   ?
    -   box dyn error
    -   [ ] expect, unwrap
-   [x] generic types
    - `impl <T> MyStruct<T>`
-   [ ] trait
    -   [x] default
    -   [x] trait fn input
    -   [x] trait bound, `+`, `where`
    -   [ ] trait object
        -   [ ] where
    - TODO: return impl from func
    -   From and Into
    -   fmt::Display
-   [ ] lifetimes
    -   static
-   [ ] collection
    -   vector
    -   [ ] slice
    -   hash map
-   [ ] closures (TODO)
    -   fn traits (Fn, FnMut and FnOnce)
    -   where syntax, dynamic dispatch
    -   difference between func pointer and fn traits and closure
-   [x] iterators
-   [ ] mod

# TODO:

https://github.com/wh5a/rustlings-solutions/tree/main/exercises

-   raw multi line string `r#`
-   async
-   ownership
    -   clone
-   borrow
    -   reference
    -   mutable reference
    -   dereferencing `*`
    -   slice
-   lifetime
    -   https://tfpk.github.io/lifetimekata/
    -   static lifetime
    -   anonymous lifetime
-   destructing
    -   struct
-   option
    -   Some(ref val)
-   result
    -   `Box<dyn Error>`
-   pattern matching
-   iterator
-   closure, `move`, `Fn`, dynamic dispatch
-   map
-   expect, unwrap
-   mod
    -   bin and lib crates
    -   use, pub, super, use glob
-   generics
    - implementation
-   trait
    -   impl A + B
-   bounds
-   smart pointer
    -   ref cell
-   arc
-   cow
-   concurrency, thread, join, channel, mutex
-   pattern matching
-   conversions
-   associated types

# app

-   closure - newton's method
-   new_address = hash(0xFF, sender, salt, bytecode)
-   spinner
- rust clojure immutable vec
