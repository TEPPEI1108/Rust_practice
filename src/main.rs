use rand::Rng;
use std::io;
use std::cmp::Ordering;


fn main() {
    println!("Guess the number");

    let secret_number = rand::rng().random_range(1..101);
    let mut count_number = 0;

    loop {
        println!("Please input your guess : ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, Err(_) => continue,
        };

        println!("You guessed :{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => { println!("Too small"); count_number += 1}
            Ordering::Greater => { println!("Too big"); count_number += 1;}
            Ordering::Equal => {
                println!("YOU WIN!!!!!");
                println!("You guessed in {count_number} times.");
                break;
            }
        }
    }
}
