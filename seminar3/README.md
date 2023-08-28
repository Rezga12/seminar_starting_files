## Seminar 3: Traits, Generics and Lifetimes

In this seminar we covered quite a few concepts about how to use structs and implement traits in rust. 
we also talked about generics and created a basic Idea about what they are used for. To be honest
lifetimes are really complicated aconcept and sometimes borrow checker make it hard for lifetimes to work
even for experienced rustaceans. For now what we covered is enough to develop easy programs and most importantly
write Solana smart contracts!
Lets look through the topics we covered on the seminar:

* Generics
  * generic methods
  * generics for structs and enums
  * Trait bounds for generic arguments
* Traits
  * Defining traits
  * implementing traits for different structs
  * foreign trait implementation rules
* lifetimes
  * how variable lifetimes work
  * defining custom lifetime parameters
  * lifetime annotations in struct definitions
  * lifetime elision rules

### Exercises
I couldn't come up with the exercises exactly matching the concepts we covered, but I tried my best.

### exercise 1: derive macro
The compiler is capable of providing basic implementations for some traits via the #[derive] attribute. 
For more info, please visit [here](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html).
basically some popular traits have default implementations supported by macros, which works on almost
all types if it doesn't requre som special handling.

in an article linked below are explanations of how the default implemetations of these traits(Copy, Debug, etc.) work.

your task in this exercise is to read the article and then add some attributes to specified lines
in `./src/derive_macro.rs` file to make it compile and work as expected.


### exercise 2: sum

The easiest exercise ever on generics. your job is to implement sum function to work
all the numeric types. write your solution in `./src/derive_macro.rs`


### exercise 3: traits
This one is also an easy exercise on traits, you just have to implement the trait for two types
and make the given code compile and work as expected. write your solution in `./src/traits.rs`

## exercise 4: robot name
I took this exercise from somewhere so I'll just copy-paste the instructions:

Manage robot factory settings.

When a robot comes off the factory floor, it has no name.

The first time you turn on a robot, a random name is generated in the format of two uppercase letters followed by three digits, such as RX837 or BC811.

Every once in a while we need to reset a robot to its factory settings, which means that its name gets wiped. The next time you ask, that robot will respond with a new random name.

The names must be random: they should not follow a predictable sequence. Using random names means a risk of collisions. Your solution must ensure that every existing robot has a unique name.

write your solution in `src/robot_name.rs`

