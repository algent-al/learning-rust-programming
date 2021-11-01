fn main() {

    // If Expressions
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than 0");
    }

    // Handling multiple conditions with "else if"
    let nr = 6;

    if nr % 4 == 0 {
        println!("nr is divisible by 4");
    } else if nr % 3 == 0 {
        println!("nr is divisible by 3");
    } else if nr % 2 == 0 {
        println!("nr is divisible by 2");
    } else {
        println!("nr is not divisible by 4, 3 or 2");
    }

    // Using "if" in a "let" Statement
    let condition = true;
    let num = if condition { 5 } else { 6 };
    println!("The value of num is: {}", num);
}
