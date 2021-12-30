use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // Create the file hello.txt if it doesn't exist
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
        other_error => {
            panic!("Problem opening the file: {:?}", other_error)
            }
        },    
    };

    // Cleaner version using closures
    let _file = File::open("world.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("world.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    // unwrap_in_action();
    // expect_in_action();
}

// unwrap() method call panic! if unwrap.txt file doesn't exist
fn unwrap_in_action() {
    let _f = File::open("unwrap.txt").unwrap();
}

// expect() works same as unwrap() but we chose the error message 
fn expect_in_action() {
    let _f = File::open("expect.txt").expect("Failed to open expect.txt file");
}
