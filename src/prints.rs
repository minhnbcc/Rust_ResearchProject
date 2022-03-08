pub fn run() {
    // Println to console
    println!("Hello, world!This is the first print line in Rust");

    //Basic Formatting println()
    println!("{} is from {}", "Mandy", "Cruise");

    //Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Nick", "Mass", "code"
    );

    //Named Arguments
    println!(
        "{name} likes to play {activity}",
        name = "John",
        activity = "Baseball"
    );

    //Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    //Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    //Basic math
    println!("10 + 10 = {}", 10 + 10);
}
