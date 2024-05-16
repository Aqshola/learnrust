use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Guess the number");
    println!("Please input your number");

    let secret_number=rand::thread_rng().gen_range(1..=10);

    
    
    loop {
        let mut guess= String::new();
    
        io::stdin().read_line(&mut guess).expect("Failed to read line");
    
        let guess:u32= guess.trim().parse().expect("Please Type a Number");
        
        match  guess.cmp(&secret_number) {
            Ordering::Less=>println!("Too Small"),
            Ordering::Greater=>println!("Too Big"),
            Ordering::Equal=>{
                println!("You Win!");
                break;
            }
        }
    }

    
    
}
