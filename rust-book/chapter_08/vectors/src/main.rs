fn main() {

    // Explicitly declaring the type
    let _v: Vec<i32> = Vec::new();
    
    // Letting Rust infer the type
    let v_infered = vec![1, 2, 3];
    println!("The first element of vector v_inferred is {}", &v_infered[0]);

    // Updating a Vector
    let mut v_mutable = Vec::new();

    v_mutable.push(5);
    v_mutable.push(6);
    v_mutable.push(7);
    v_mutable.push(8);

    // Dropping a Vector Drops Its Elements
    {
        let scoped_v = vec![1, 2, 3, 4];
        // do stuf with scoped_v for example printing an element of the Vector
        println!("scoped_v[0]={}", &scoped_v[0]);
    } // <- v goes out of scope and is freed here

    // Reading Elements of Vectors
    // Method 1 of accessing a value in a vector with indexing syntax
    let third: &i32 = &v_mutable[2];
    println!("Method 1: The third element is {}", third);

    // Method 2 of accessing a value in a vector with 'get' method.
    match v_mutable.get(2) {
        Some(third) => println!("Method 2: The third element is {}", third),
        None => println!("Method 2: There is no third element."),
    }

    match v_mutable.get(4) {
        Some(forth) => println!("Method 2: The forth element is {}", forth),
        None => println!("Method 2: There is no forth element."),
    }

    // Iterating over the Values in a Vector
    println!("Printing each element of a vector");
    // Immutable vector loop
    let v1 = vec![100, 32, 57];
    for i in &v1 {
        println!("{}", i);
    }

    // Mutable vector loop
    let mut v2 = vec![25, 54, 38];
    for i in &mut v2 {
        // Add 50 to every element of the vector
        *i += 50;
        println!("{}", i); // Print the result
    }

    // Using an Enum to Store Multiple Types
    #[derive(Debug)] // To print this Enum
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in row {
        println!("{:?}", i);
    }
}