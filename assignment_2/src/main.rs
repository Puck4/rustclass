// Step 1: Define the secret number (hard-coded)
const SECRET_NUMBER: i32 = 42;

// Step 2: Implement the check_guess function
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0 // Guess is correct
    } else if guess > secret {
        1 // Guess is too high
    } else {
        -1 // Guess is too low
    }
}

fn main() {
    // Step 3: Initialize variables
    let mut guesses = 0;
    let mut guess = 50; // Starting guess (you can change this)

    // Step 4: Loop to repeatedly ask for guesses
    loop {
        // Step 5: Call the check_guess function
        let result = check_guess(guess, SECRET_NUMBER);

        // Step 6: Print feedback based on the result
        if result == 0 {
            println!("Congratulations! You guessed the secret number {} in {} guesses.", SECRET_NUMBER, guesses);
            break;
        } else if result == 1 {
            println!("Too high! Try again.");
            guess -= 1; // Adjust guess downwards
        } else {
            println!("Too low! Try again.");
            guess += 1; // Adjust guess upwards
        }

        // Increment guesses counter
        guesses += 1;
    }
}

