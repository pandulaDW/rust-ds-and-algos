use std::collections::HashMap;

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

fn _valid_palindrome(s: String) -> bool {
    let is_palindrome = |chars: &[u8]| -> bool {
        for i in 0..(chars.len() / 2) {
            if chars[i] != chars[chars.len() - (i + 1)] {
                return false;
            }
        }
        return true;
    };

    let input = s.as_bytes();
    let mut i = 0;
    let mut end = input.len() - 1;

    while i < end {
        if input[i] != input[end] {
            let s1 = &input[(i + 1)..(end + 1)];
            let s2 = &input[i..end];
            if is_palindrome(s1) || is_palindrome(s2) {
                return true;
            }
            return false;
        }
        i += 1;
        end -= 1;
    }

    return is_palindrome(&input);
}

fn _two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut solution = Vec::<i32>::with_capacity(2);
    let compliments: HashMap<i32, i32> = nums.iter().zip(0..).map(|(v, i)| (*v, i)).collect();

    for (i, v) in nums.into_iter().enumerate() {
        let compliment = target - v;
        if let Some(val) = compliments.get(&compliment) {
            if *val != i as i32 {
                solution.push(i as i32);
                solution.push(*val);
                break;
            }
        }
    }

    return solution;
}

fn _add_binary(a: String, b: String) -> String {
    let add_two_binary_digits = |d1: u8, d2: u8, carry: u8| {
        let sum = (d1 - 48) + (d2 - 48) + carry;
        match sum {
            0 => (0, 0),
            1 => (1, 0),
            2 => (0, 1),
            _ => (1, 1),
        }
    };

    let nums_1 = a.as_bytes();
    let nums_2 = b.as_bytes();
    let mut solution: Vec<u8> = Vec::with_capacity(nums_1.len().max(nums_2.len()) + 1);
    let mut i: i32 = 0;
    let mut carry = 0;

    while nums_1.len() > i as usize || nums_2.len() > i as usize {
        let result = add_two_binary_digits(
            *nums_1
                .get(((nums_1.len() as i32) - (i + 1)) as usize)
                .unwrap_or(&48),
            *nums_2
                .get(((nums_2.len() as i32) - (i + 1)) as usize)
                .unwrap_or(&48),
            carry,
        );
        solution.push(result.0 + 48);
        carry = result.1;
        i += 1;
    }

    if carry == 1 {
        solution.push(49);
    }

    solution.reverse();

    String::from_utf8(solution).unwrap()
}

fn _remove_adjacent_duplicates(s: String) -> String {
    let word = s.as_bytes();
    let mut stack = Vec::new();

    for ch in word {
        if stack.is_empty() {
            stack.push(*ch);
            continue;
        }
        if *ch == *stack.last().unwrap() {
            stack.pop();
        } else {
            stack.push(*ch);
        }
    }

    return String::from_utf8(stack).unwrap();
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

    #[test]
    fn test_valid_palindrome() {
        assert_eq!(true, _valid_palindrome("aba".to_string()));
        assert_eq!(true, _valid_palindrome("abca".to_string()));
        assert_eq!(false, _valid_palindrome("abc".to_string()));
        assert_eq!(true, _valid_palindrome("deeee".to_string()));
        assert_eq!(false, _valid_palindrome("cxcaac".to_string()));
    }

    #[test]
    fn test_two_sum() {
        assert_eq!(vec![0, 1], _two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], _two_sum(vec![3, 2, 4], 6));
        assert_eq!(vec![0, 1], _two_sum(vec![3, 3], 6));
    }

    #[test]
    fn test_remove_adjacent_duplicates() {
        assert_eq!("b", _remove_adjacent_duplicates("aab".to_string()));
        assert_eq!("abd", _remove_adjacent_duplicates("aaabccddd".to_string()));
        assert_eq!("", _remove_adjacent_duplicates("aa".to_string()));
        assert_eq!("", _remove_adjacent_duplicates("baab".to_string()));
        assert_eq!("m", _remove_adjacent_duplicates("mississippi".to_string()));
        assert_eq!("ay", _remove_adjacent_duplicates("azxxzy".to_string()));
        assert_eq!("ca", _remove_adjacent_duplicates("abbaca".to_string()));
    }

    #[test]
    fn test_add_binary() {
        let a = "11".to_string();
        let b = "1".to_string();
        assert_eq!("100", _add_binary(a, b));
        assert_eq!("10101", _add_binary("1010".to_string(), "1011".to_string()));
    }
}
