pub fn run(){
    // Params sorted
    println!("{0} likes to {1}, {0} is {2}.", "Chokri", "code", "happy");

    // named variables
    println!(
        "{name} likes to play {activity}.",
        name = "Chokri",
        activity = "BasketBall"
    );

    // Formatting string
    println!("Bin: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // Debugging a tuplet
    println!("{:?}", (12, true, "Hello"));

    // Printing an operation
    println!("{} + {} = {}", 5, 7 , 5+7);
}