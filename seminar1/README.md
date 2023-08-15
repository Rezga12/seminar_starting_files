## Seminar 1: The Simplest of The Rust Programs

Today we covered the most basic concepts of the rust programming language. 
More advanced stuff _(and probably stuff that makes rust different from other languages)_ 
will be coming shortly on the next few seminars. Until then, we can go through what we covered _(or should have had covered)_ so far:


* variable binding, `let` - keyword
* shadowing
* scopes
* tuples,
* functions, return types
* `if`- statement, control flow
* `match` statement
* `enums`
* `structs`
* accessing fields in a struct
* accessing
* importing types.
* declaring methods on our own types
* variable mutability
* using `mut` keyword
* shadowing vs `mut`
* immutability of struct fields
* constants
* enums
* patterns and matching _(...if we have time)_

The greater portion of the code in slides was taken from this sweet article: [A half-hour to learn Rust](https://fasterthanli.me/articles/a-half-hour-to-learn-rust)
Also after covering all those simple but important topics I highly recommend reading _(or just running through)_ the corresponding
chapters in **The Rust Programming Language** book:

* [Chapter 2: Programming a guessing game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)
* [Chapter 3: Common programming concepts](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html)

#### Now to the exercises!

let's review the project structure first! \
In `/src` directory there are 4 `.rs` files. you don't have to touch `lib.rs`, it just contains tests for the exercises.\
what you need to edit are the other three files. each contains one function to be implemented and each of these functions contain
`unimplemented!()` macro, which you should remove or comment out before proceeding with your solution. 

#### building the project

first, from your terminal access the `./seminar1` folder, the command may differ from depending on OS.
once in the project folder type:\
`> cargo build`\
to run tests for all the exercises, type:\
`> cargo test`\
to run tests only for one specific exercise:\
`> cargo test -- --test <exercise_name>`\
ex:\
`> cargo test -- --test hello_world`


### Exercise 1: Hello World! _(who would have thought!)_

This is one of the simplest possible functions in rust:
```rs
pub fn hello() ->  &'static str {
    unimplemented!()
}
```

but even looking at this simple function we can notice that rust
is somewhat different from other programming languages: for example what does that weird `'static` keyword do? why is the
returning type called `str` and not `String`? why does a method `unimplemented!()` have an exclamation mark at the end?\
Time will come, and we will answer all those questions, but for now, just try your best to return string `"Hello, World!"` correctly.


### Exercise 2: Reverse String

Well, this one is also the simplest of the exercises, we just have to reverse a string and return it, 
```rs
pub fn reverse(input: &str) -> String {
    unimplemented!("Write a function to reverse {input}");
}
```

looking at this code, we can see that rust not only has a `&str` type, it also has a bigger `String` type, what is the difference?
or maybe they are the same? we will cover those types in the next seminars, but if you are too interested, you can look through [this 
chapter](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html) of the book. _(but I don't recommend to, it might be too hard for a begginer)_
Well that's it for the assignment explanation. now try to reverse a string or should I sat **str**, good luck!


### Exercise 3: Giga second

The implementation of this function might be even easier than the last. The date `struct` is passed into your function, all you
need to do is to add one billion-second period to that date and return it back.\
This exercise can be written in two lines, but main reason for its existence is to show students how external crates are
used. In this specific exercise we are using `time`, crates are like libraries in other languages, and we will be talking about them a lot in
the next seminars, for now we just need to find out how we can create new instance of `DateTime` struct, or how to add an arbitrary
number of seconds to the already existing one. good luck again!.


