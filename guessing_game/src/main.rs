use std::io;
use rand::Rng;//Something related to triats
use std::cmp::Ordering; //An enum similar to the Error with finite defined values called variants

fn main() {
    println!("Guess the number !");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    //println!("The secret number is {}", secret_number);
    let mut iteration_count = 0;

    loop{
        
        println!("Please input your guess.");

        let mut guess = String::new(); //The new is a 'static' method of the String Type rather than a 'dynamic' method of an instance of the type
        
        io::stdin()//This function stdin() returns a Stdin instance and this istance has the function items like read_line, expect etc
            .read_line(&mut guess)//Reads whatever the user types in at the screen and stores at the guess variable and also returns a io::Result type.
            .expect("Failed to read line");//this is panic if something bad occurs and it is a part of io::Result type
    
        let guess: u32 = guess.trim().parse().expect("Please type a number !");
    
        println!("You guessed: {}", guess);
        iteration_count+= 1;

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You Win!! with the Interation count = {}", iteration_count);
                break;
            },
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too Small"),
        }
    }

}
