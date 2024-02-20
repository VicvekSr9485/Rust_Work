fn main() {
    let condition = true;
    let number = if condition { 5 } else {4};

    if number > 7 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    let b = [10, 20, 30, 40, 50];

    for element in b {
        println!("the value is: {element}");
    }

    for number in (1..10).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
