use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];
    //let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a={}", a[0]);

    let months = [
                "Janary", "February", "Mars", "April",
                 "May", "June", "July", "August",
                 "September", "October", "November", "December"
    ];
    println!("First month is {}", months[0]);

    let b = [3; 5];
    let b1 = b[0];
    let b2 = b[1];
    println!("b1={}, b2={}", b1, b2);

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
