fn binary_search(arr: &[i32], data: i32) -> Option<usize> {
    let mut min: usize = 0;
    let mut max = arr.len();

    while min < max {
        let mid = min + (max - min) / 2; // Prevents overflow

        if data == arr[mid] {
            return Some(mid);
        } else if data > arr[mid] {
            // Ignore left half
            min = mid + 1;
        } else {
            // Igore right half
            max = mid;
        }
    }

    None
}

fn main() {
    // Input array
    let arr = [2, 3, 5, 7, 11, 13, 17, 19, 23];
    let target = 11;

    match binary_search(&arr, target) {
        Some(index) => println!("Result: {}", index),
        None => println!("Element not found"),
    }
}
