use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){

    let rand_num = rand::thread_rng().gen_range(1..=100);

    loop{
        let mut guess = String::new();

        println!("\nEnter your guess(1-100): ");
    
        io::stdin().read_line(&mut guess).expect("Failed to read line!!");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => if num<=0 || num>100 {0} else {num},
            Err(_) => {println!("Enter Valid Number from 1 to 100"); continue},
        };

        if guess == 0 {
            println!("Enter a valid Number between 1-100.");
        }
        else{

            println!("You guessed : {guess}");

            match guess.cmp(&rand_num) {
                Ordering::Less => println!("Too Low!😒"),
                Ordering::Greater => println!("Too High!🤥"),
                Ordering::Equal => {
                    println!("You win!🥳");
                    break;
                }
            }
        }
    }
}