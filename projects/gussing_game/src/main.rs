use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");


    // let secret_number = rand::Rng::gen_range(&mut rand::thread_rng(), 1..10);
    let secret_number = rand::thread_rng().gen_range(0..=100);

    println!("The secret number is: {}", secret_number);
       
    loop {
        println!("Please input your guess."); 
        let mut guess = String::new(); // mut means mutable, let means variable

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };  
        // match is like a switch statement, but it's more powerful, it can match on any type, 
        // in here it mathces on the result of parse() to see if it's Ok or Err
        // trim() removes whitespace, parse() converts string to number, expect() handles error
        println!("You guessed: {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
