fn main() {
    // Ownership and Functions
    {
        let s = String::from("hello!"); // s comes into scope
        takes_ownerships(s);            // value of s moves info the function
        // Here, s is no longer valid.

        let x = 5;                      // x comes into scope
        makes_copy(x);                  // x woud move into the function
        
        println!("x={} is OK", x);      // but i32 is copy, so it is OK to use x afterwards 
    } // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.

    {
        let s1 = gives_ownership();         // gives_ownership moves its return value into s1
        let s2 = String::from("hello rust");     // s2 comes into scope
        let s3 = takes_and_gives_back(s2);  // s2 is moved to takes_and_gives_back, which also moves it return value into s3
        println!{"s1={}, s3='{}', sorry s2 is moved to s3", s1, s3};
    } // Here, s1 and s3 goes out of scope and is droped, s2 mas moved, so nothing happens.

    // Returning ownership of parameters
    {
        let s1 = String::from("calculate");
        let (s2, len) = calculate_length(s1);
        println!("The length of '{}' is {}.", s2, len);
    }
}

fn takes_ownerships(some_string: String) {  // some_string comes into scope
    println!("{}", some_string);            // do stuff with some_string
} // Here, some_string gets out of scope and "drop" is called. The backing memory is fried.

fn makes_copy(some_integer: i32) {          // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer get out of scope. Nothing special happens.

fn gives_ownership() -> String {             // gives_ownership will move its return value into the function that calls it
    let some_string = String::from("yours"); // some_string comes into scope
    some_string                              // some_string is returned and moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String {   // a_string comes into scope
    a_string                                            // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();    // len() returns the length of a String
    (s, length)              // Return a tuple with (s, length)
}