use std::io;

fn main() {
    println!("Welcome to FORTUNE TELLER");
    println!("(Warning: if you use invalid inputs the program will fail)");
    println!("Here your fortune will be toooolldd. How are you feeling today?");
    let feeling = input();

    println!("Sooo you are feeling {feeling} are you now?");
    println!("What's your name dear user?");
    let name = input();

    println!("Hi {name}!")
}

fn input() -> String {
    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Some error occurred while taking your input, please restart the program");
    match text.strip_suffix("\n") {
        Some(text) => String::from(text),
        None => panic!("Idk what happened, try again pls")
    }
}