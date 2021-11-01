fn main() {
    let x = 2.5;        // f64
    let y: f32 = 3.4;   // f32
    println!("x={}, y={}", x, y);

    //addition
    let sum = 5 + 10;
    println!("sum={}", sum);

    //subtraction
    let diff = 95.5 - 60.6;
    println!("diff={}", diff);

    // multiplication
    let mult = 4 * 25;
    println!("mult={}", mult);

    // division
    let div = 30.0 / 0.5;
    println!("div={}", div);

    // remainder
    let rem = 43 % 5;
    println!("rem={}", rem);

    // Boolean
    let t = true;
    let f: bool = false;
    println!("t={}, f={}", t, f);

    // Characters
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("c={}, z={}, heart_eyed_cat={}", c, z, heart_eyed_cat);
}
