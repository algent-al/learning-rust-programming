// Adding the attribute to derive the Debug trait and printing the Rectangle instance using debug formatting
#[derive(Debug)]
struct Rectangle {
    width: u32,
    heigh: u32,
}

fn main() {
    // Calculate the area of rectangle using single variables
    let width1 = 30;
    let heigh1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area1(width1,heigh1)
    );

    // Refactoring with Tuples
    let rect1 = (30, 50);

    println!(
        "The area of the rect1 is {} square pixels.",
        area2(rect1)
    );

    // Refactoring with structs and Debug
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        heigh: 50, // heigh: dbg!(50),
    };

    // Another way to print out a value using the Debug format
    dbg!(&rect2);
    
    // Pretty printing of a Rectangle instance
    println!("rect2 is {:?}", rect2); // {:#?} to include curly bracktes

    println!(
        "The area of the rect2 is {} square pixels.",
        area3(&rect2)
    );
}

// Single variables example
fn area1(width: u32, heigh: u32) -> u32 {
    width * heigh
}

// The tuple example
fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// The struct example
fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.heigh
}