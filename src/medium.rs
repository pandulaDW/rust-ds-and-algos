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
}
