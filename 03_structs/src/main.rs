// include functionality to print out debugging information
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 35, height: 50 };
    println!(
        // the reason we use ampersand here is because we dont want are function
        // to take ownsership of struct rect1, case we need it after println...
        "The area of the rectangle is {} square pixels", area(&rect1)
    );
    // check traits and custom behaviours -> chapter 10
    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
