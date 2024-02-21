use std::io;

fn main() {
    println!("Enter temperature in Fahrenheit: ");

    loop {
        let mut scale = String::new();

        io::stdin()
            .read_line(&mut scale)
            .expect("Failed to read line");

        let scale: f32 = match scale.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please enter a valid number!");
                continue;
            }
        };

        let celsius = (scale - 32.0) * 5.0 / 9.0;

        println!("Temperature in Celsius: {:.2}", celsius);
        break;
    }
}
