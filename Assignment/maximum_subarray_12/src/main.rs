// Q-12

//function to find the maximum sub array
//The logic uses the Kadane's Algorithm
fn max_sub_array(nums: &[i32]) -> i32 {
    let mut max = nums[0];
    let mut curr_max = nums[0];

    for &num in nums.iter().skip(1) {
        curr_max = num.max(num + curr_max);
        max = curr_max.max(max);
    }

    max
}

fn main() {
    let nums = vec![1,2,3,-6,7,8,-15,11,12,43];
    let result = max_sub_array(&nums);
    println!("Maximum subarray sum: {}", result);
}
    