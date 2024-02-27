#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_fit(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 25,
        height: 35,
    };
    let rect2 = Rectangle {
        width: 40,
        height: 55,
    };
    let rect3 = Rectangle {
        width: 20,
        height: 30,
    };

    println!(
        "The area of the rectangle is {} square pixels.", 
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    println!("Can rect1 fit into rect2? {}", rect1.can_fit(&rect2));
    println!("Can rect1 fit into rect3? {}", rect1.can_fit(&rect3));
}