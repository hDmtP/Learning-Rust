use std::io;

fn main() {
    println!("Hello there, Guess a number");
    println!("Please enter ur input");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Didn't get that");
    println!(
        "You have guessed: {}", guess
    );
}
