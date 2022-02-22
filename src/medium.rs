use std::collections::HashMap;

fn _remove_consecutive_kdigits(num: String, k: i32) -> String {
    let s: Vec<char> = num.chars().collect();
    let mut min = i32::MAX;

    for i in 0..s.len() {
        // break the loop if the word cannot be chunked further
        match s.get(i..(k as usize + i)) {
            Some(_) => (),
            None => break,
        };

        // left prefix
        let mut left = match s.get(0..i) {
            Some(v) => Vec::from(v),
            None => vec![],
        };

        // right suffix
        let mut right = match s.get((k as usize + i)..) {
            Some(v) => Vec::from(v),
            None => vec![],
        };

        // append the two affixes
        left.append(&mut right);

        // check the remaining number
        if let Ok(remaining_number) = left.into_iter().collect::<String>().parse::<i32>() {
            if remaining_number < min {
                min = remaining_number;
            }
        }
    }

    return format!("{:?}", if min == i32::MAX { 0 } else { min });
}

fn _remove_adjacent_duplicates(s: String, k: i32) -> String {
    let word = s.as_bytes();
    let mut stack: Vec<(u8, i32)> = Vec::new();

    for ch in word {
        if stack.is_empty() {
            stack.push((*ch, 1));
            continue;
        }

        let last = *stack.last().unwrap();

        if *ch == last.0 && last.1 == k - 1 {
            for _ in 0..(k - 1) {
                stack.pop();
            }
        } else if *ch == last.0 && last.1 < k - 1 {
            stack.push((*ch, last.1 + 1));
        } else if *ch != last.0 {
            stack.push((*ch, 1));
        }
    }

    return String::from_utf8(stack.into_iter().map(|v| v.0).collect()).unwrap();
}

fn _remove_sub_folders(folder: Vec<String>) -> Vec<String> {
    let create_paths = |input: &str| {
        let folder_names: Vec<&str> = input.split("/").skip(1).collect();
        let mut paths = Vec::with_capacity(folder_names.len());

        for i in 0..folder_names.len() - 1 {
            paths.push(format!("/{}", &folder_names[0..i + 1].join("/")));
        }

        return paths;
    };

    let mut path_map: HashMap<String, u8> = HashMap::new();
    folder.iter().for_each(|path| {
        path_map.insert(path.clone(), 1);
    });

    folder
        .into_iter()
        .filter(|v| {
            let all_folder_paths = create_paths(v);
            for path in all_folder_paths {
                if path_map.contains_key(&path) {
                    return false;
                }
            }
            return true;
        })
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_remove_consecutive_kdigits() {
        assert_eq!(
            "1219",
            _remove_consecutive_kdigits("1432219".to_string(), 3)
        );
        assert_eq!("1", _remove_consecutive_kdigits("10001".to_string(), 4));
        assert_eq!("200", _remove_consecutive_kdigits("10200".to_string(), 1));
        assert_eq!(
            "412853",
            _remove_consecutive_kdigits("435512853".to_string(), 3)
        );
        assert_eq!("0", _remove_consecutive_kdigits("10".to_string(), 2));
    }

    #[test]
    fn test_remove_adjacent_duplicates() {
        assert_eq!(
            "aa",
            _remove_adjacent_duplicates("deeedbbcccbdaa".to_string(), 3)
        );
        assert_eq!("abcd", _remove_adjacent_duplicates("abcd".to_string(), 2));
        assert_eq!(
            "ps",
            _remove_adjacent_duplicates("pbbcggttciiippooaais".to_string(), 2)
        );
    }

    #[test]
    fn test_remove_sub_folders() {
        let input = vec![
            "/a/b/c".to_string(),
            "/a/b/ca".to_string(),
            "/a/b/d".to_string(),
        ];
        assert_eq!(
            vec![
                "/a/b/c".to_string(),
                "/a/b/ca".to_string(),
                "/a/b/d".to_string(),
            ],
            _remove_sub_folders(input)
        );
    }
}
