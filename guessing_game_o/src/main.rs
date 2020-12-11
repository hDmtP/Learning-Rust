use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number");
    let secrete_number = rand::thread_rng().gen_range(1,101);
    println!("Please input  tyour guess: ");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Didn't get that");
    println!("You guessed: {} ", guess);
    print!("Original number: {}", secrete_number);
}
