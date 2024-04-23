// Q-1

fn is_palindrome(s: &str) -> bool {
    let s = s.chars().filter(|c| c.is_alphanumeric()).collect::<String>().to_lowercase();
    let reversed = s.chars().rev().collect::<String>();
    s == reversed
}

fn main() {
    let test_cases = vec!["A man, a plan, a canal, Panama", "racecar", "hello"];

    for test_case in test_cases {
        if is_palindrome(test_case) {
            println!("'{}' is a palindrome", test_case);
        } else {
            println!("'{}' is not a palindrome", test_case);
        }
    }
}
