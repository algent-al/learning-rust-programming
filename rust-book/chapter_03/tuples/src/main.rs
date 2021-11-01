fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    
    let y = tup.1;
    println!("y={}", y);

    let (a, b, c) = tup;
    println!("a={}, b={}, c={}", a, b, c);
}
