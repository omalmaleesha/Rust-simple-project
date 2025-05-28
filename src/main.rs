use std::io;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("⏳ Countdown Timer");
    println!("Enter countdown time in seconds:");

    // Get user input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Parse input to integer
    let seconds: u64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("❌ Please enter a valid number.");
            return;
        }
    };

    // Countdown loop
    for i in (1..=seconds).rev() {
        println!("Time remaining: {} seconds", i);
        sleep(Duration::from_secs(1));
    }

    println!("⏰ Time's up!");
}
