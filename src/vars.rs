pub fn run() {
    let name = "Codie";

    //const
    let mut age = 25;
    println!("My name is {} and I am {}", name, age);

    age = 40;
    println!("My name is {} and I am {}", name, age);

    //Define constant
    const ID: i32 = 001;

    println!("ID: {}", ID);

    //Assign multple vars
    let (my_name, my_age) = ("Brad", 25);
    println!("{} is {}", my_name, my_age);
}
