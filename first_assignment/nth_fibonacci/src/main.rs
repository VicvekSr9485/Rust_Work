fn fibonacci(n: u32) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}



fn main() {
    let n = 7;
    println!("The {}th Fibonacci number is: {}", n, fibonacci(n));
}
