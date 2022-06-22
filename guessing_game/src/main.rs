use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess a number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100); // same as 1..101
    println!("Please input your guess: ");

    loop{

        let mut guess = String::new(); // mutable variable
        // let geuss = String::new(); // this would be unmutable
    
        //  std::io::stdin
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to Start");
    
        println!("You guessed: {}", guess);
        
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
    
        // match is like switch
        match  guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("you win!");
                break;
            },
            Ordering::Greater => println!("too big!"),
            Ordering::Less => println!("Too small")
        }
        // println!("the number was: {}", secret_number);
    }

}
