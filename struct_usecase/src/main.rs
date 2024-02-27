fn main() {
    let rect1 = Rectangle {
        width: 25,
        height: 35,
    };

    let rectangle = area_struct(&rect1);

    println!("The area of the rectangle is {} square pixels.", &rectangle);
}

//Area of a rectangle using simple logic
//fn area(width: u32, height: u32) -> u32 {
//    width * height
//}

// Area of a rectangle using tuple
//fn area_tuple(dimensions: (u32, u32)) -> u32 {
//  dimensions.0 * dimensions.1
//}

// Refactoring with structs
struct Rectangle {
    width: u32,
    height: u32,
}
fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}