// &'static is a "lifetime specifier", something you'll learn more about later
// str - is a return type, which indicates that we are returning string literal
// unimplemented! macro is used here just to make compiler happy
// type cargo test in the terminal when you are done.
pub fn hello() ->  &'static str {
    unimplemented!()
}
