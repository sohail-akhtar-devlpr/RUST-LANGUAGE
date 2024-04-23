// Q-3

//function to find the shortest word among the words.
fn shortest_word_3(s: &str) -> Option<&str> {
    let mut shortest_word_start = 0;
    let mut shortest_word_end = s.len();
    let mut current_word_start = 0;
    let mut current_word_end = 0;
    let mut found_word = false;

    for (i, c) in s.char_indices() {
        if c.is_whitespace() {
            if found_word {
                if current_word_end - current_word_start < shortest_word_end - shortest_word_start {
                    shortest_word_start = current_word_start;
                    shortest_word_end = current_word_end;
                }
                found_word = false;
            }
        } else {
            if !found_word {
                current_word_start = i;
                found_word = true;
            }
            current_word_end = i + c.len_utf8();
        }
    }

    if found_word && current_word_end - current_word_start < shortest_word_end - shortest_word_start {
        shortest_word_start = current_word_start;
        shortest_word_end = current_word_end;
    }

    if shortest_word_end > shortest_word_start {
        Some(&s[shortest_word_start..shortest_word_end])
    } else {
        None
    }
}

fn main() {
    let input = "Man in the middle attack";
    match shortest_word_3(input) {
        Some(shortest_word) => println!("The shortest word is: {}", shortest_word),
        None => println!("No words found in the input string"),
    }
}
