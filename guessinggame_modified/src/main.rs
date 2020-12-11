use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");
    let secrete_number = rand::thread_rng().gen_range(1,101);
    loop{
    let mut guess = String::new();
    io::stdin()
    .read_line(&mut guess)
    .expect("Didn't get that");
    // let guess: u32 = guess.trim().parse().expect("Bylat! Type a number");
    let guess: u32 = match guess.trim().parse() {
        Ok(number) => number,
        Err(_) => continue
    };
    
    
        println!("Please input  tyour guess: ");
        match guess.cmp(&secrete_number){
            Ordering::Greater => println!("BIg"),
            Ordering::Less => println!("small"),
            Ordering::Equal => {println!("You Win!");
            break;
        }
    }
    println!("You guessed: {} ", guess);
}
println!("Original number: {}", secrete_number);
}
