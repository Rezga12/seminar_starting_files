## Seminar 2: Working With Ownership

Today we covered rust's one of the main concepts: Ownership. Let's go through
what we covered:

* Copying and Cloning
* `Box<T>` type and heap allocation
* Concept of **Move**
* Deallocation Principle
* Ownership Rules
* References and Borrowing
* Aliasing and Mutability

The Code snippets and examples on the lecture slides were taken from rust book's
[Chapter 4: Understanding Ownership](https://rust-book.cs.brown.edu/ch04-00-understanding-ownership.html)
Here's the other version of the chapter: https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html

To build and test your seminar project use instructions from [Seminar 1](../seminar1/README.md)

### Exercises

#### implementing drop function

This is more like an ideological exercise. It ensures that you understand how drop works in Rust.
your goal is to make a variable be dropped before it goes out of scope.
you should write your code in `src/drop_function.rs` file

#### make me compile
in this exercise you are given a small rust functions which doesn't compile due to 
ownership or borrow checker rules. your task is to make minimal changes in code so that
it compiles and produces the desired output. the instructions are written as
comments in `src/make_me_compile.rs` file.

#### pass the seeds
In this exercise your main goal is to produce `&[&[&[]]]` type and pass it to the specified function.
On the first glance it might be a weird type, so we should get acquinted with it by some practice.
you can use any means necessary to produce data of this type.

#### sublist
this is a simple challenge, where we will learn a few new concepts in rust.
you are Given two lists determine if the first list is contained within the second list, if the second list is contained
within the first list, if both lists are contained within each other or if none of these are true.

Specifically, a list A is a sublist of list B if by dropping 0 or more elements from the front of B and 0 or more elements 
from the back of B you get a list that's completely equal to A.

Examples:

* A=**[1, 2, 3]**, B = **[1, 2, 3, 4, 5]**, A is a `sublist` of B
* A = **[3, 4, 5]**, B = **[1, 2, 3, 4, 5]**, A is a `sublist` of B
* A = **[3, 4]**, B = **[1, 2, 3, 4, 5]**, A is a `sublist` of B
* A = **[1, 2, 3]**, B = **[1, 2, 3]**, A is `equal` to B
* A = **[1, 2, 3, 4, 5]**, B = **[2, 3, 4]**, A is a `superlist` of B
* A = **[1, 2, 4]**, B = **[1, 2, 3, 4, 5]**, A is not a `superlist` of, sublist of or equal to B

in `src/sublist.rs` function you are given a `sublist` function which looks like this:

```rs
pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
```
first of all, pay attention to the type `T`, it's a type parameter which signifies that
out sublist function is **Generic**.\
the notion of `T: PartialEq` means that our type `T` implements an equality operator.
this syntax expresses the generic parameter bounds, which we will touch a little in the
next lectures.

other than that everything should be clear by looking at the code, if something is hard to understand feel free to ask in the class.

to run tests type `cargo test` in the terminal\
to run a specific test type `cargo test -- --test <name of a specific test>`