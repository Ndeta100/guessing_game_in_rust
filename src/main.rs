use rand::Rng;
use std::io;
fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(0..50);
    println!("secret number : {}", secret_number);
    println!("Please input your guess");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    print!("You guessed: {}", guess);
}
