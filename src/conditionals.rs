// Conditionals - Used to check the conditionos somehing and act

pub fn run() {
    let age: u8 = 22;
    let check_id: bool = true;
    let knows_person_of_age = true;

    //If /Else
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && check_id {
        println!(
            "Bartender: Sorry you have to leave. Your age {:?} is illegal to drink",
            age
        );
    } else {
        println!("Bartender: I'll need to see your ID");
    }

    // Shorthand If (ternary operator)
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is Of Age: {}", is_of_age);

    // match check_id {
    //     false => println!("Bartender: Sorry you have to leave."),
    //     true => println!("Bartender: What would you like to drink?"),
    // }
}
