use std::io;

fn main() {
    println!("Generate the nth Fibonacci number");

    println!("\nPlease enter nth term");
    let mut nth = String::new();
    io::stdin()
        .read_line(&mut nth)
        .expect("Failed to read line");
    
    let n: i32 = nth.trim().parse().expect("Please insert number");

    let mut i: i32 = 0;
    
    // set f_1, f_2 and f_n; i64  for n > 46
    // set f_1, f_2 and f_n; i128 for n > 92
    let mut f_n: i64 = 0;
    let mut f_1: i64 = 0;
    let mut f_2: i64;

    while i < n {
        if i == 0 {
            println!("i={}, F({})={}",i, i, f_n);
            f_n += 1;
            i += 1;
        }
        else {
            f_2 = f_1;
            f_1 = f_n;
            f_n = f_2 + f_1;
            i += 1;
        }
        println!("i={}, F({})={}",i, i, f_n);
    }
}