fn main() {
    // Mutable string
    let _s = String::new();

    let data = "initial contents";
    let s = data.to_string();

    println!("s is: {}", s);

    // the method above also works on a literal directly:
    // let s = "initial contents".to_string();

    // String::from and to_string do the same thing
    let st = String::from("initial contents");
    println!("{}", st); // To remove warnings

    // Strings are UTF-8 encoded, so we can include any properly encoded data in them
    let _hello = String::from("السلام عليكم");
    let _hello = String::from("Dobrý den");
    let _hello = String::from("Hello");
    let _hello = String::from("שָׁלוֹם");
    let _hello = String::from("नमस्ते");
    let _hello = String::from("こんにちは");
    let _hello = String::from("안녕하세요");
    let _hello = String::from("你好");
    let _hello = String::from("Olá");
    let _hello = String::from("Здравствуйте");
    let _hello = String::from("Hola");

    // Updating a String by appending a String slice
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("{} {}", s1, s2);
    println!("s2 is {}", s2);

    // Adding one character to a String value using push
    let mut s3 = String::from("lo");
    s3.push('l');
    println!("s3 is {}", s3);

    // Concatenation with the + Operator or the format! Macro
    concatenate_strings_with_plus_operator();

    // concatenate multiple strings
    concatenate_multiple_strings();

    // Methods for Iterating Over Strings
    itirating_over_strings_with_chars_method();
    itirating_over_strings_with_bytes_method();
}

// Concatenation with the + Operator or the format! Macro
fn concatenate_strings_with_plus_operator() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("s3 is '{}'", s3);
}

// concatenate multiple strings
fn concatenate_multiple_strings() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s4 = s1 + "-" + &s2 + "-" + &s3;
    println!("s4 is '{}'", s4);

    // For more complicated string combining, we can use the format! macro
    let s1 = String::from("tic");
    let s5 = format!("{}-{}-{}", s1, s2, s3);
    println!("s5 is '{}'", s5);
}


// Methods for Iterating Over Strings
fn itirating_over_strings_with_chars_method() {
    let hello = String::from("こんにちは");
    for c in hello.chars() {
        println!("{}", c);
    }
}

fn itirating_over_strings_with_bytes_method() {
    let hello = String::from("こんにちは");
    for b in hello.bytes() {
        println!("{}", b);
    }
}