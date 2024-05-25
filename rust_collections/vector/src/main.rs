use std::collections::HashMap;

fn median_mode(numbers: &Vec<i32>) -> (f64, i32) {
    let mut sorted_numbers = numbers.clone(); // a copy to sort.
    sorted_numbers.sort();

    // calculation the median
    let median = if sorted_numbers.len() % 2 == 0 {
        let mid1 = sorted_numbers[sorted_numbers.len() / 2 - 1];
        let mid2 = sorted_numbers[sorted_numbers.len() / 2];
        (mid1 as f64 + mid2 as f64) / 2.0
    } else {
        sorted_numbers[sorted_numbers.len() / 2] as f64
    };

    // calculate the mode
    let mut freq_map = HashMap::new();
    for &num in numbers.iter() {
        *freq_map.entry(num).or_insert(0) += 1;
    }

    let mut mode = sorted_numbers[0];
    let mut max_count = 0;

    for (&num, &count) in freq_map.iter() {
        if count > max_count {
            max_count = count;
            mode = num;
        }
    }

    (median, mode)

}

fn main() {
    let numbers = vec![2, 8, 4, 9, 5, 4, 5, 4, 3];

    let (median, mode) = median_mode(&numbers);

    println!("Median: {}", median);
    println!("median: {}", mode);
}
