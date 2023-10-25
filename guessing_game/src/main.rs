use std::io; // this import the input/output library. it comes from the standard library a.k.a std

fn main() {

    println!("Guess the Number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess)

    // the .expect method is a result method that handles ok and error
    .expect("Failed to read line");

    println!("You guessed: {guess}");
    
}
