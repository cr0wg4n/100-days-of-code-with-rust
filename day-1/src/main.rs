use std::io;

const THIS_IS_A_CONSTANT: i32 = 23;

fn exploring_variables () {
    let mut x = 10;
    println!("The value of x is: {x}");
    x = 23;
    println!("The new value of x is: {x}");
    println!("Constats can be used globally {THIS_IS_A_CONSTANT}");

    let y = 10;
    {
        let y = y + 2;
        println!("shadowing y {y}");
    }
    println!("original y value: {y}");

    let mut spaces = "    ";
    spaces = "demo";
    println!("print string {spaces}");
}

fn main() {
    println!("Guess the number!");
    println!("Please input your guess");
    let mut input = String::new();
    io::stdin().read_line(& mut input).expect("Failed to read line");
    println!("You guessed: {input}");


    exploring_variables();
}

