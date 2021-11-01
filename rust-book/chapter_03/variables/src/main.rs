fn main() {
    // variabels
    let mut xx = 5;
    println!("The value of xx is: {}", xx);
    xx = 6;
    println!("The value of xx is: {}", xx);

    // Constants
    const MONTH:i32 = 12;
    println!("A year have {} months", MONTH);

    // Shadowing by changing the value ans still keep it immutable
    let x = 5;
    let x = x + 1;

    {
        // This is inner scope
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);

    // Shadowing by using same name but different type
    let spaces = "    ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);
}
