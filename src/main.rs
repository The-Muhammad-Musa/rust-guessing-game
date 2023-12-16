mod pretty_printer;

use std::io;
use std::io::Write;
use rand::Rng;
use crate::pretty_printer::{cold, cool, hot, warm};

fn main() {
    // Storing wins and losses counts
    let mut user_wins: i32 = 0;
    let mut user_losses: i32 = 0;
    // Storing the answer as a var
    let ans: i32 = generate_answer();
    // Display those rules!
    display_rules();
    // Now we run the game's round, passing it the answer. Returning a bool on whether they won or not
    let win: bool = game_time(ans);
    // Check if user won or not
    if win == true {
        user_wins += 1;
    } else {
        user_losses += 1;
    }
    // Now we are going to loop through the game until the user doesn't want to play anymore
    while play_again() {
        // Now we repeat the above but in our loop!
        let ans: i32 = generate_answer();
        let win: bool = game_time(ans);
        if win == true {
            user_wins += 1;
        } else {
            user_losses += 1;
        }
    }
    // We hit this once the user is done playing
    println!("You won {} times and lost {} times!", user_wins, user_losses);
    println!("Thank you for playing, have a great day!")
}

fn display_rules() -> () {
    // Just telling the user the rules :3
    println!("Welcome to the Number Guessing Game!\nThe rules are pretty simple:");
    println!("1. You have to guess a number between 1 and 100");
    println!("2. You have 5 tries to guess the number");
    println!("3. If you guess the number correctly, you win!");
    println!("4. After each incorrect guess you will be told whether you were hot, cold, cool, or warm");
}

fn generate_answer() -> i32 {
    // Creating a mutable range variable which is a thread range type from the rand crate
    let mut rng = rand::thread_rng();
    // Create our answer variable which is a range of 1 to 100
    let answer = rng.gen_range(1..101);
    // Send that answer back buddy boi
    return answer;
}

fn game_time(ans: i32) -> bool {
    // If returns true, won this round. Else, lost this round.
    let mut user_lives: i32 = 5;
    // Now we need to iterate through the loop 3 times
    while user_lives > 0 {
        // Ask the user for their guess, print macro instead of println to put answer on same line
        print!("Please enter your guess: ");
        io::stdout().flush().unwrap();
        // Create a mutable variable to store the user's guess
        let mut user_guess = String::new();
        // Read the user's guess and store it in the user_guess variable
        io::stdin().read_line(&mut user_guess).expect("Failed to read line");
        // Convert the user's guess to an integer
        let user_guess: i32 = match user_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // Check if the user's guess is correct
        if user_guess == ans {
            println!("Good job! The answer was indeed {}! You won!", ans);
            return true;
        } else {
            // Check if the user's guess is within 5 of the answer
            if user_guess >= ans - 10 && user_guess <= ans + 10 {
                println!("{}", hot("is hot!", user_guess));
            } else if user_guess >= ans - 20 && user_guess <= ans + 20 {
                println!("{}", warm("is warm!", user_guess));
            } else if user_guess >= ans - 30 && user_guess <= ans + 30 {
                println!("{}", cool("is cool!", user_guess));
            } else {
                println!("{}", cold("is cold!", user_guess));
            }
            // Decrement the user's lives
            user_lives -= 1;
        }
    }
    println!("You are out of lives, sorry! The answer was: {}", ans);
    false
}

fn play_again() -> bool {
    // Now we are going to ask the user whether they want to play again or not, returning true if
    // they want to play again, false if they are done.
    print!("Would you like to play again? (y/n) ");
    io::stdout().flush().unwrap();
    let mut user_choice = String::new();
    io::stdin().read_line(&mut user_choice).expect("Failed to read line");
    return if user_choice.trim() == "y" || user_choice.trim() == "yes" {
        true
    } else {
        false
    }
}
