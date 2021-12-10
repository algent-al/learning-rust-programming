use std::collections::HashMap;

fn main() {
    // Creating a new HashMap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    // Accessing Values in a Hash Map
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{}'s scores is {:?}", team_name, score);

    // Iterate over each key/value pair in a hash map using a for loop
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Updating a HashMap
    overwriting_a_value();
    only_insert_value_if_key_has_no_value();
    update_value_based_on_old_value();
}

// Vector of tuples to HashMap
fn _vec_tup_hm() {
    let teams = vec![String::from("Blue"), String::from("Red")];
    let initial_scores = vec![10, 50];

    // we can use <_, _> because Rust will infer the type as <String, i32>
    let mut _scores: HashMap<_, _> = 
        teams.into_iter().zip(initial_scores.into_iter()).collect();
}

// Hash Maps and Ownership
fn _hm_ownership() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point
}

// Updating a HashMap
fn overwriting_a_value() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);
}

fn only_insert_value_if_key_has_no_value() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Red")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}

fn update_value_based_on_old_value() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
