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

fn _reverse_string(s: &mut Vec<char>) {
    let word_len = s.len();

    for i in 0..word_len / 2 {
        let temp = s[i];
        s[i] = s[word_len - (i + 1)];
        s[word_len - (i + 1)] = temp;
    }
}

fn _longest_common_prefix(strs: Vec<String>) -> String {
    let mut prefix = String::new();

    for (i, ch) in strs[0].chars().enumerate() {
        let prefix_included = strs.iter().skip(1).all(|v| {
            let cur = match v.chars().nth(i) {
                Some(v) => v,
                None => return false,
            };
            return cur == ch;
        });
        if prefix_included {
            prefix.push(ch);
        } else {
            break;
        }
    }

    return prefix;
}

fn _fizz_buzz(n: i32) -> Vec<String> {
    return (1..n + 1)
        .map(|val| {
            if val % 3 == 0 && val % 5 == 0 {
                return "FizzBuzz".to_string();
            } else if val % 3 == 0 {
                return "Fizz".to_string();
            } else if val % 5 == 0 {
                return "Buzz".to_string();
            } else {
                return format!("{}", val);
            }
        })
        .collect();
}

fn _excel_sheet_column_number(column_title: String) -> i32 {
    let mut sum = 0;

    for (i, ch) in column_title.chars().into_iter().enumerate() {
        let index = (ch.to_digit(36).unwrap() - 9) as i32;
        sum += 26_i32.pow((column_title.len() - (i + 1)) as u32) * index;
    }

    return sum;
}

fn _is_palindrome(s: String) -> bool {
    let filtered = s
        .chars()
        .into_iter()
        .filter_map(|ch| {
            if ch.is_alphanumeric() {
                return Some(ch.to_ascii_lowercase());
            }
            return None;
        })
        .collect::<Vec<char>>();

    for i in 0..(filtered.len() / 2) {
        if filtered[i] != filtered[filtered.len() - (i + 1)] {
            return false;
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

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
        let mut input = "hello".to_string().chars().collect(); // odd case
        _reverse_string(&mut input);
        assert_eq!(vec!['o', 'l', 'l', 'e', 'h'], input);

        input = "worlds".to_string().chars().collect(); // even case
        _reverse_string(&mut input);
        assert_eq!(vec!['s', 'd', 'l', 'r', 'o', 'w'], input);

        input = vec!['s'];
        _reverse_string(&mut input);
        assert_eq!(vec!['s'], input);
    }

    #[test]
    fn test_longest_common_prefix() {
        let input = vec![
            "flow".to_string(),
            "flower".to_string(),
            "flight".to_string(),
        ];

        assert_eq!("fl", _longest_common_prefix(input));
    }

    #[test]
    fn test_fizz_buzz() {
        assert_eq!(vec!["1", "2", "Fizz"], _fizz_buzz(3));
        assert_eq!(vec!["1", "2", "Fizz", "4", "Buzz"], _fizz_buzz(5));
        assert_eq!(
            vec![
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz"
            ],
            _fizz_buzz(15)
        );
    }

    #[test]
    fn test_excel_sheet_column_number() {
        assert_eq!(2, _excel_sheet_column_number("B".to_string()));
        assert_eq!(27, _excel_sheet_column_number("AA".to_string()));
        assert_eq!(28, _excel_sheet_column_number("AB".to_string()));
        assert_eq!(703, _excel_sheet_column_number("AAA".to_string()));
    }

    #[test]
    fn test_is_palindrome() {
        assert_eq!(false, _is_palindrome("race a car".to_string()));
        assert_eq!(true, _is_palindrome("race e car".to_string()));
        assert_eq!(
            true,
            _is_palindrome("A man, a plan, a canal: Panama".to_string())
        );
        assert_eq!(true, _is_palindrome("".to_string()));
    }
}
