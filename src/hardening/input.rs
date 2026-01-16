use std::io::{self, Write};
use rpassword::read_password;

pub fn secure_prompt(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    match read_password() {
        Ok(input) => input.trim().to_string(),
        Err(_) => String::new(),
    }
}
