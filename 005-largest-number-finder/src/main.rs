fn largest_number_finder (input_array: &[i32]) -> Option<i32> {
    if let Some(&first) = input_array.first() {
        let mut largest_number = first;

        for &num in input_array.iter().skip(1) {
            if num > largest_number {
                largest_number = num;
            }
        }

        Some(largest_number)
    } else {
        None
    }
}

// Alternative way
// fn largest_number_finder(input_array: &[i32]) -> Option<i32> {
//     input_array.iter().max().copied()
// }

fn main() {
    // Input array
    let numbers = [3, 5, 7, 2, 8, -1, 4, 10];

    match largest_number_finder(&numbers) {
        Some(x) => println!("The largest number: {x}"),
        None => println!("The array is empty."),
    }

    let empty_array: [i32; 0] = [];

    match largest_number_finder(&empty_array) {
        Some(x) => println!("The largest number: {x}"),
        None => println!("The array is empty."),
    }
}
