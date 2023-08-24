fn print_string_twice() {
    let x = String::from("hello, world");
    let y = x;
    println!("{},{}",x,y);
}

// Don't modify code in this!
// fn print_string_twice_with_helper_function() {
//     let s1 = String::from("hello, world");
//     let s2 = take_ownership(s1);

//     println!("{}", s2);
// }

// Only modify the code below!
fn take_ownership(s: String) {
    println!("{}", s);
}

fn dont_use_clone(){
    let x = (1, 2, (), "hello".to_string());
    let y = x.clone();
    println!("{:?}, {:?}", x, y);
}

fn string_in_tuple() {
    let t = (String::from("hello"), String::from("world"));

    let _s = t.0;
 
    // Modify this line only, don't use `_s`
    println!("{:?}", t);
}