fn simple_function () {
    let hello = "Hello";

    println!(hello);

    // we need to drop hello here by implementing drop function

    let world = "World";

    println!(world);
}

// TODO implement drop function
fn drop(){

}