use std::io;
 
fn main() {
    println!("Temperature Convertions!");

    println!("Enter 'c' to covert from Fahrenheit to to Celsius");
    println!("Enter 'f' to covert from Celsius to to Fahrenheit");
    let mut conv = String::new();
    io::stdin()
        .read_line(&mut conv)
        .expect("Failed to read line");
    
    // println!("You selected {}!", conv);

    let conv: char = conv.trim().parse().expect("Please insert 'c' or 'f'");

    println!("Please enter a value");
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let temp: f32 = temp.trim().parse().expect("Please insert a number");

    if conv == 'c' {
        println!("Converting {}°C!", temp);
        println!("The temperature in Fahrenheit is: {}°F!", c_to_f(temp));
    }
    else if conv == 'f' {
        println!("Converting {}°F!", temp);
        println!("The temperature in Celsius is: {}°C!", f_to_c(temp));
    }
    else {
        println!("Please select 'c' or 'f'");
    }
}

// Celsius to Fahrenheit conversion function
fn c_to_f(temp:f32) -> f32 {
    temp * 1.8 + 32.0
}
 
// Fahrenheit to Celsius conversion function
fn f_to_c(temp:f32) -> f32 {
    (temp - 32.0) / 1.8
}
