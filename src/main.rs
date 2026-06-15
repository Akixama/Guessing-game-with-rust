//Printing the user input

// use std::io;

// fn main(){
//     println!("Guess the number!");

//     println!("Please input your guess:");

//     let mut guess = String::new();

//     io::stdin()
//                 .read_line(&mut guess)
//                 .expect("Failed to read line");

//     println!("You guessed {guess}");
// }


//PRINTING A RANDOM NUMBER FROM 1-100
use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main(){
    println!("Guessing Game");

    //Gotten my random number
    let guessing_number = rand::thread_rng().gen_range(1..=10);
    // println!("that is {guessing_number}");

    //Getting user input
loop {
    
    let mut guess = String::new();
    
    println!("Guess The Number");
    
    //Allowing user to input
    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Put in a Number");

println!("You guessed: {guess}");

match guess.cmp(&guessing_number){
    Ordering::Less => println!("It is too small"),
    Ordering::Greater => println!("It is too big"),
    Ordering::Equal => {
        println!("You win!");
        break;
    }
    }
    }

}