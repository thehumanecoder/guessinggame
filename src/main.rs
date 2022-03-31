use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::*;

fn main() {
    println!("Guess the number !");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let parse_result = guess.trim().parse();

        let guess : u32 = match parse_result {
            Ok(num) =>num,
            Err(_)=>continue
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too small !!".red()),
            Ordering::Greater => println!("{}","Too big !!".red()),
            Ordering::Equal => {
                println!("{}","Eureka , You win !!".green());
                break;
            }
        }
    }
}
