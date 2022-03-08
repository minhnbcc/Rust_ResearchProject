//Funcitons - Used to store blocks of code for re-use

pub fn run() {
    greeting("Hi", "Minh");

    // Bind function values to variables
    let get_sum = add(5, 5);
    println!("Sum: {}", get_sum);

    // Closure
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C Sum: {}", add_nums(3, 3));
    println!("--------------------------------");
    println!("How old are you?");
    println!("I am {:?} year-old", age(2022, 1987));
    println!("--------------------------------");
    bmi(70.00, 1.75);
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

fn age(current_year: i32, year_born: i32) -> i32 {
    current_year - year_born
}

fn bmi(weight: f32, height: f32) {
    let bmi = weight / (height * height);
    if bmi < 18.5 {
        print!("Your BMI {} is considered underweight", bmi)
    } else if bmi >= 18.5 && bmi <= 24.9 {
        print!("Your BMI {} is considered normal weight", bmi)
    } else if bmi >= 25.0 && bmi <= 29.9 {
        print!("Your BMI {} is considered overweight", bmi)
    } else {
        print!("Your BMI {} is considered obesity", bmi)
    }
}
