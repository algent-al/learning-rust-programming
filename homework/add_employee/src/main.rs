use std::{collections::HashMap, io};

fn main() {
    println!("Enter e a command like \"Add <person> to <Department>\"");
    directory();
}

// This function is going to create a hash map
// The Key will be the directory which is the department
// The value will be the vector containing the employee names
fn directory() {
    let mut employee_directory = HashMap::new();

    loop {
        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read command");

        let command: &str = command.trim();
        println!("You entered {}", command);
        let mut iter = command.split_whitespace();
        let person = match iter.nth(1) {
            Some(p) => p,
            None => {
                println!("1:Please enter a valid command");
                continue;
            }
        };

        // let person = iter.nth(1).expect("Invalid command");
        // let depart = iter.nth(3).expect("Invalid command");
        let department = match iter.nth(1) {
            Some(d) => d,
            None => {
                println!("2: Please enter a valid command");
                continue;
            }
        };

        let employees = employee_directory.entry(String::from(department)).or_insert(vec![]);
        employees.push(String::from(person));
        println!("Employee directory: {:?}", employee_directory);
    }
}
