// Q-9

//function to reverse the string
fn reverse_string(s: &str) -> String {
    if s.is_empty() {
        return String::from(s);
    }

    let mut chars: Vec<char> = s.chars().collect();
    let mut left = 0;
    let mut right = chars.len() - 1;

    while left < right {
        // Swap characters at left and right pointers
        let temp = chars[left];
        chars[left] = chars[right];
        chars[right] = temp;
        // Move pointers towards each other
        left += 1;
        right -= 1;
    }

    chars.into_iter().collect()
}

fn main() {
    let input = "A Big Elephant, Hippo, and Rhino";
    let reversed = reverse_string(input);
    println!("Reversed string: {}", reversed);
}
