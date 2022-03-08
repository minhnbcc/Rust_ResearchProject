pub fn run() {
    //Default is "i32";
    let x = 1;

    //Default is "f64";
    let y = 2.5;

    //Add explicit type
    let z: i64 = 454545454545454;

    //find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    //Boolean
    let is_active = true;

    //Get boolean from exppression
    let is_greater: bool = 10 < 5;

    let a1 = "a";
    //emoji smiley face
    let face = "\u{1F600}";

    //emoji waive hand
    let waive_hand = "\u{1f44b}";

    println!(
        "{:?}",
        (x, y, z, is_active, is_greater, a1, face, waive_hand)
    );
}
