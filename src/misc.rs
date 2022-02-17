fn _length_of_last_world(input: String) -> i32 {
    let mut len_word = 0;
    for ch in input.chars().rev() {
        if ch == ' ' && len_word == 0 {
            continue;
        }
        if ch != ' ' {
            len_word += 1;
        } else {
            break;
        }
    }
    return len_word;
}

fn _reverse_string(input: String) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    let word_len = chars.len();

    for i in 0..word_len / 2 {
        let temp = chars[i];
        chars[i] = chars[word_len - (i + 1)];
        chars[word_len - (i + 1)] = temp;
    }

    return chars.into_iter().collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_last_world() {
        let mut input = "hello  world".to_string();
        assert_eq!(5, _length_of_last_world(input));

        input = "hello world    ".to_string();
        assert_eq!(5, _length_of_last_world(input));

        input = "  hello w  ".to_string();
        assert_eq!(1, _length_of_last_world(input));

        input = "".to_string();
        assert_eq!(0, _length_of_last_world(input));

        input = "hello unicode: චිද".to_string();
        assert_eq!(3, _length_of_last_world(input));
    }

    #[test]
    fn test_reverse_string() {
        let mut input = "hello".to_string(); // odd case
        assert_eq!("olleh", _reverse_string(input));

        input = "worlds".to_string(); // even case
        assert_eq!("sdlrow", _reverse_string(input));

        input = "".to_string();
        assert_eq!("", _reverse_string(input));
    }
}
