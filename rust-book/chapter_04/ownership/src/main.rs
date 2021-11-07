fn main() { 
    // Example 1
    { // s is not valid here, it's not yet declared
        let s = "Hello!!!"; // s is valid from this point forward
        println!("{}",s);   // doing stuff with s
    } // scope of s is now over, and is no longer valid

    // Example 2
    {
        let mut s = String::from("Hello"); // This way the string is mutable
        s.push_str(", world"); // push_str() appends a literal to a String
        println!("{}",s);
    }

    // Ways Variables and Data Interact
    // Example 1: Move
    {
        let x = 5;
        let y = x; // These variables are pushed into stack
        println!("{}",y);

        // If values are pushed into heap they are no longer still valid after being shallow copied
        let s1 = String::from("hello!");
        let s2 = s1; // These variables are pushed into heap
        println!("{}",s2); // here s1 is invalidated. We get error if we try to print s1.
    }

    // Example 2: Clone
    {
        let s1 = String::from("hello!");
        let s2 = s1.clone();
        println!("s1 = {}, s2 = {}",s2, s1); // now s1 is valid
    }

    // Stack-Only Data
    // Example 1: Copy
    {
        // If values are pushed into stack they are still valid after being shallow copied
        let x = 5;
        let y = x;

        println!("x = {}, y = {}", x, y);

        let tup: (u32, bool, f64, char, i32) = (117, true, 3.14, 'C', 5);
        let tup_clone = tup;
        let (tup_x, tup_y, tup_z, _, _) = tup;  // _ to skip the value
        let (tup_clone_x, tup_clone_y, tup_clone_z, _, _) = tup_clone;

        println!("tup_x = {}, tup_clone_x = {}", tup_x, tup_clone_x);
        println!("tup_y = {}, tup_clone_y = {}", tup_y, tup_clone_y);
        println!("tup_z = {}, tup_clone_z = {}", tup_z, tup_clone_z);
    }
}
