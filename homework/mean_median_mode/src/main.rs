use std::collections::HashMap;

fn main() {
    let numbers = vec![1, 6, 2, 3, 4, 4, 4, 5, 1, 0];
    println!("Vector numbers: {:?}", numbers);
    
    let average = mean(&numbers);
    println!("Average is {}", average);

    let median = median(&numbers);
    println!("Median is {}", median);

    let my_mode = mode(&numbers);
    println!("Mode is {}", my_mode);
}

fn mean(numbers: &[i32]) -> f64 {
    // Get the sum of the numbers in the vector and devide by the lengh of the vector
    let mut sum = 0.0;
    for num in numbers {
        sum += *num as f64
    }
    sum / numbers.len() as f64
}

fn median(numbers: &[i32]) -> f64 {
    // Get a copy of numbers vec and sort it
    let mut sorted_numbers = numbers.to_vec();
    sorted_numbers.sort();
    println!("Sorted numbers: {:?}", sorted_numbers);
    let middle = sorted_numbers.len() / 2;
    // Get the average of two middle numbers if the vector has even numbers
    if sorted_numbers.len() % 2 == 0 {
        return mean(&vec![sorted_numbers[middle], sorted_numbers[middle - 1]]);
    }
    sorted_numbers[middle] as f64
}

fn mode(numbers: &[i32]) -> i32 {
    let mut map = HashMap::new();
    for num in numbers {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }
    println!("Map of ocurrencies {:?}", map);
    let mut max_value = 0;
    let mut mode = 0;
    for (key, value) in map {
        if value > max_value {
            max_value = value;
            mode = *key;
        }
    }
    mode
}
