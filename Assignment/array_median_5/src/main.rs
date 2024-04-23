// Q-5

fn array_median(arr: &[i32]) -> Option<f64> {
    let len = arr.len();
    if len == 0 {
        return None;
    }
    
    if len % 2 == 0 {
        // If array length is even, return the average of the middle two elements
        let mid_right = len / 2;
        let mid_left = mid_right - 1;
        Some((arr[mid_left] as f64 + arr[mid_right] as f64) / 2.0)
    } else {
        // If array length is odd, return the middle element
        Some(arr[len / 2] as f64)
    }
}

fn main() {
    let arr = [1, 2, 3, 4, 5,6];
    match array_median(&arr) {
        Some(median) => println!("Median of the array is: {}", median),
        None => println!("Array is empty"),
    }
}
