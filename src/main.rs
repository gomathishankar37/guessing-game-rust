use std::io;
// use IO library
use rand::Rng;
// use Random library
use std::cmp::Ordering;
// enum giving result of 2 things being compared (Less: -1, Equal: 0, Greater: 1)
use colored::*;
// Use Colored library

fn main() {
    println!("{}", "Guess the number!!!".cyan());

    let secret_number = rand::thread_rng().gen_range(1, 101);
    //random number from 1 to 100

    let mut tries: u32 = 0;

    loop {
        println!("{}", "Input your Guess!!!! ".yellow());
        let mut guess = String::new();
        // initially empty guess

        // prompts
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // get input and handle exception

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // guess variable is shadowed to change the Data Type from String to u32
        //trim and parse the input, if int flow, if error continue the iteration

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("{}", "Too small".red());
                tries += 1;
            }
            Ordering::Greater => {
                println!("{}", "To large".red());
                tries += 1;
            }
            Ordering::Equal => {
                print!("{}", "You Win!!!!! in ".green());
                println!("{} attempts", tries);
                break;
            }
        };
        // match the comparison result enum to L, G or E and print outputs
    }
}
