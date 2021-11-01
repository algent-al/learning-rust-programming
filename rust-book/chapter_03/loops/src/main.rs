fn main() {
    // Repeating code with "loop"
    loop {
        println!("again!");
        break;
    }

    // Using "continue" and "break"
    let mut count = 0;
    // 'count_up eshte emri i loop-it qe e emerton vete
    'count_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 8 {
                break;
            }
            if count == 3 {
                break 'count_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);

    // Returning values from loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    // Conditional loops with "while"
    let mut num = 3;
    while num != 0 {
        println!("Num is {}!", num);
        num -= 1;
    }
    println!("End!");

    // Looping Through a Collection with "for"
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("Index {} is {}",index, a[index]);
        index += 1;
    }
    println!("Index is {}",index);

    let b = [5, 10, 15, 20, 25];
    for element in b {
        println!("The value is {}", element);
    }

    for n in (1..4).rev() {
        println!("{}", n);
    }
    println!("The end!")
}
