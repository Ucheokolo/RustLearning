use std::{io, cmp:: Ordering }; // this import the input/output library. it comes from the standard library a.k.a std
use rand::Rng;


fn main() {

    print_values();

    let generated_number = rand::thread_rng().gen_range(1..=100);

    guess_fn(generated_number);
    
    
}


fn guess_fn(secrete_digit : u32) {

    println!("Guess the Number!");

    println!("Please input your guess.");

    let secrete_number: u32 = secrete_digit;


    
    loop {
        let mut guess = String::new(); //initial guess line, this will bring a data type mismatch with secret number generator.

        io::stdin()
        .read_line(&mut guess)
    
        // the .expect method is a result method that handles ok and error
        .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("please type a number!");

        // I dprecated the above code for the one below to enable handling of errors when a user inputs an alphabet instead of a number. the match function handles this problem..

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secrete_number) {
        Ordering::Less => println!("too small, You guessed: {guess} try Again!"),
        Ordering::Greater => println!("too big, You guessed: {guess} try Again!"),
        Ordering::Equal => {println!("Congratulations!! You Win!!! You guessed: {guess}");
        break;
        },
        // Instead of using the control flow to break, we use a curly braces within the coorect optio for the match function and include the break prompt to the code block.
        }
        // if secrete_number == guess {
        //     break;
        // }
    }


    println!("Correct number is {secrete_number}");

}

fn print_values(){
    let x = 5;
    let y = 10;
    println!("x={x} and y + 2 = {}", y + 2);
}
