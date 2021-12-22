use std::io;

const VOWELS: [&str;6] = ["a", "e", "i", "o", "u", "y"];

fn main() {
    println!("Enter a word to convert in pig latin!");

    let mut original_word = String::new();
    io::stdin()
        .read_line(&mut original_word)
        .expect("Failed to read line");

    let original_word: String = original_word.trim().parse().expect("Enter a word");

    let pig_latin_version = convert_to_pig_latin(&original_word);
    println!("{} in pig latin is {}", original_word, pig_latin_version);
}

fn convert_to_pig_latin(word: &str) -> String {
    /* Convert strings to pig latin. 
    The first consonant of each word is moved to the end of the word and “ay” is added, 
    so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the 
    end instead (“apple” becomes “apple-hay”). */
    let (first, rest) = &word.split_at(1);
    let is_vowel = VOWELS.contains(&first);
    if is_vowel {
       return format!("{}-{}", word, "hay");
    }
    return format!("{}-{}ay", rest, first);
}
