fn main() {
    let s1 = String::from("hello world");
    let hello = &s1[0..5];
    let world = &s1[6..11];
    println!("{} {}", hello, world);

    let s2 = String::from("hello");
    let len = s2.len();

    let slice = &s2[..2]; // this is equal to: let slice = &s[0..2];
    let end = &s2[3..len];  // this is equal to let end = &t[3..];
    println!("'{}' '{}'", slice, end);

    let s = String::from("first word");
    let word = first_word(&s);
    // s.clear(); // this empties the String, making it equal to ""
    println!("The first word is '{}'", word);

    let my_string = String::from("my string");

    // `first_word` works on slices of `String`s, whether partial or whole
    let w1 = first_word(&my_string[0..2]);
    let w2 = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent to whole slices of `String`s
    let w3 = first_word(&my_string);

    println!("w1='{}', w2='{}', w3='{}'", w1, w2, w3);

    let my_string_literal = "string literal";

    // `first_word` works on slices of string literals, whether partial or whole
    let w4 = first_word(&my_string_literal[0..6]);
    let w5 = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already, this works too, without the slice syntax!
    let w6 = first_word(my_string_literal);

    println!("w4='{}', w5='{}', w6='{}'", w4, w5, w6);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes(); // convert string to an array of bytes

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
