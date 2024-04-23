// Q-6

//function to find the longest common prefix
fn longest_common_prefix(strs: &[String]) -> String {
    let mut result = String::new();

    // Sort the array of strings
    let mut sorted_strs = strs.to_vec();
    sorted_strs.sort();

    // Convert the first and last strings to char arrays
    let first = sorted_strs[0].chars().collect::<Vec<char>>();
    let last = sorted_strs[sorted_strs.len() - 1].chars().collect::<Vec<char>>();

    // Iterate over the characters of the first string
    for i in 0..first.len() {
        // If the character at the same index in the first and last string is different, break the loop
        if first[i] != last[i] {
            break;
        }
        // Append the character to the result string
        result.push(first[i]);
    }
    result
}

fn main() {
    let strs = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];
    let common_prefix = longest_common_prefix(&strs);
    println!("Longest common prefix: {}", common_prefix);
}
