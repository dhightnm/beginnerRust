use std::io;
use rand::Rng;
use std::any::type_name;
use std::cmp::Ordering;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut try_count = 0; 
   loop { 
        println!("Please input your guess");
        
        let mut guess = String::new();
        
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if guess.trim().to_lowercase() == "quit" {
                    break;
                } else {
                println!("Please Input a valid number");
                println!("You guess: {}", type_of(&guess));
                continue;
                }
            }
        };

        println!("You guessed: {}", guess);
        try_count += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! It took you {} tries", try_count);
                break;
            }
        }
        if try_count >= 3 {
        println!("The secret number is: {}", secret_number);
        }
    }  
}
