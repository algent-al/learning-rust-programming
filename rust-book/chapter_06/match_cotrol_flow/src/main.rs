#[derive(Debug)] //  so we can inspect the state in a minute
enum UsState {
    Alabama,
    _Alaska,    // Add _ to resolve warnings because we are not using them
    _Chicago,
    _Florida,
    /* --snip-- */
}

enum Coin {
    Penny,
    _Nickel,    // Add _ to resolve warnings because we are not using them
    _Dime,
    Quarter(UsState),
}

fn main() {
    // The match Control Flow Operator
    let penny = Coin::Penny;
    println!("The value in cents of a penny is {}", value_in_cents(penny));

    // Patterns that Bind to Values
    let alabama = Coin::Quarter(UsState::Alabama);
    println!("The value in cents of a quarter is {}", value_in_cents(alabama));

    // Matching with Option<T>
    let five = Some(5);
    let x = plus_one(five);
    let none = plus_one(None);

    println!("five={:?}", five);
    println!("x={:?}", x);
    println!("none={:?}", none);

    // Concise Control Flow with if let
    print_only_fives(Some(1));
    print_only_fives(Some(5));
    print_only_fives(Some(7));
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::_Nickel => 5,
        Coin::_Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn print_only_fives(x: Option<u8>) {
    if let Some(5) = x {
        println!("5")
    }
}