fn main() {
    println!("Hello, world!");

    // Add here another function
    another_function();

    // Add here a third function
    third_function(5, 6);

    // The fourth function
    // Characters should be between ''
    my_height(1.78, 'm');

    // Statements and expressions
    let a = 5;

    println!("The value of a is {}", a);

    let b = {
        let a = 3;
        a + 1
    };

    println!("The value of b is {}", b);

    // Functions with return values
    let z = five();
    println!("The value of z is {}", z);

    let t = plus_one(5);
    println!("The value of t is {}", t);
}

// Define another function
fn another_function() {
    println!("Another function.");
}

// Defining the third function
// The values in parentheses are called parameters
fn third_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// Defining the fourth function
fn my_height(x: f64, h: char) {
    println!("I am {}{} tall", x, h);
}

// Defining the fifth function
fn five() -> i32 {
    5
}

// Defining sixth function
fn plus_one(t: i32) -> i32 {
    t + 1
}