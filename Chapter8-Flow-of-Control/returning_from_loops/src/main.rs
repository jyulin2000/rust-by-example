use std::io::prelude::*;

fn main() {
    let result : i32 = loop {
        print!("Please enter a number: ");
        std::io::stdout().flush().unwrap();

        let mut user_input = String::new();
         
        std::io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        match user_input.trim().parse::<i32>() {
            Ok(num) => break num,
            Err(_) => continue
        }
    };

    println!("You entered: {}", result);
}
