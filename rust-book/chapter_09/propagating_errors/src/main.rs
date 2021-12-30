use std::fs::{self, File};
use std::io::{self, Read};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let _f = File::open("hello.txt")?;
    Ok(())
}

// A function that returns errors to the calling code using match
fn _read_username_from_file_long_way() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// A Shortcut for Propagating Errors: the ? Operator
fn _read_username_from_file_short_way() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// Shorter way: Chaining method calls after the ? operator
fn _read_username_from_file_the_shorter_way() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// Shortest way: Using fs::read_to_string instead of opening and then reading the file
fn _read_username_from_file_the_shortest_way() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}