use std::{
    error::Error,
    io::{stdout, Write},
};

fn main() -> Result<(), Box<dyn Error>> {
    print!(" > ");
    stdout().flush()?;
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;
    buf = buf.trim().to_owned();
    println!();

    println!(
        "{}",
        if is_palindromic(&buf) {
            format!("{buf} is a palindrome")
        } else {
            format!("{buf} is not a palindrome")
        }
    );

    Ok(())
}

fn is_palindromic(s: &str) -> bool {
    s.to_owned() == s.chars().rev().collect::<String>()
}
