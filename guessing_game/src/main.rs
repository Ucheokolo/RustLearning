use std::{io, cmp::{self, Ordering}}; // this import the input/output library. it comes from the standard library a.k.a std
use rand::Rng;


fn main() {

    print_values();
    guess_fn();

    
    
}


fn guess_fn(){

    println!("Guess the Number!");

    println!("Please input your guess.");

    let secrete_number = rand::thread_rng().gen_range(1..=100);

    let mut guess = String::new(); //initial guess line, this will bring a data type mismatch with secret number generator.
    
    io::stdin()
    .read_line(&mut guess)
    
    // the .expect method is a result method that handles ok and error
    .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("please type a number!");

    match guess.cmp(&secrete_number) {
        Ordering::Less => println!("too small, You guessed: {guess}"),
        Ordering::Greater => println!("too big, You guessed: {guess}"),
        Ordering::Equal => println!("Congratulations!! You Win!!! You guessed: {guess}"),
    }
    

    println!("Correct number is {secrete_number}");

}

fn print_values(){
    let x = 5;
    let y = 10;
    println!("x={x} and y + 2 = {}", y + 2);
}
