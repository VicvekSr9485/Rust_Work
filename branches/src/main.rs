fn main() {
    let condition = true;
    let number = if condition { 5 } else {4};

    if number > 7 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
