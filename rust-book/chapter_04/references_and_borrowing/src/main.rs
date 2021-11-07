fn main() {
    // References and Borrowing
    let s1 = String::from("references");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);

    // Mutable references
    let mut s = String::from("mutable");

    let s1 = &s;
    let s2 = &s;
    println!("s1 = '{}', s2 = '{}'", s1, s2);
    // The scopes of the immutable references s1 and s2 end after the println! where they are last used, 
    // which is before the mutable references r1 and r2 are created.

    change(&mut s);
    println!("s = '{}'", s);

    {   // we can use curly brackets to create a new scope, allowing for multiple mutable references
        // just not simultaneous ones:
        let r1 = &mut s;
        println!("r1 = '{}'", r1);
    } // r1 goes out of scope here, so we can make a new reference with no problems
    let r2 = &mut s;
    println!("r2 = '{}'", r2);

    // Dangling References
    // let reference_to_nothing = dangle();
    let reference_to_nothing = no_dangle();
    println!("{}", reference_to_nothing);
}

// A function that calculate length but doesn't take ownership
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, nothing happens.


fn change(some_string: &mut String) {
    some_string.push_str(" references");
}

// fn dangle() -> &String {            // dangle return reference to a string
//     let s = String::from("hello");  // s is a new string
//     &s  // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away. DANGER!!!

// works because we actually return the String
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}