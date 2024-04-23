// Q-7

use std::collections::BinaryHeap;

fn kth_smallest_element_7(a: &[i32], k: usize) -> Option<i32> {
    let mut pq = BinaryHeap::new();

    // Add the first k elements to the priority queue
    for &num in a.iter().take(k) {
        pq.push(num);
    }

    // Iterate through the rest of the elements
    for &num in a.iter().skip(k) {
        // If the current element is smaller than the largest element in the heap
        // Replace the largest element in the heap with the current element
        if let Some(&top) = pq.peek() {
            if num < top {
                pq.pop();
                pq.push(num);
            }
        }
    }

    pq.peek().cloned()
}

fn main() {
    let a = [3, 1, 5, 4, 2];
    let k = 3; // Find the 3rd smallest element
    match kth_smallest_element_7(&a, k) {
        Some(kth_smallest) => println!("The {}rd smallest element is: {}", k, kth_smallest),
        None => println!("Array is too small or k is out of range"),
    }
}
