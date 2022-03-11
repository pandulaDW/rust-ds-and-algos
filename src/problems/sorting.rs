fn _bubble_sort<T: Ord>(s: &mut Vec<T>) {
    loop {
        let mut swapped = false;
        for i in 0..s.len() - 1 {
            if s[i] < s[i + 1] {
                swapped = true;
                s.swap(i + 1, i);
            }
        }
        if !swapped {
            break;
        }
    }
}

fn _selection_sort<T: Ord>(s: &mut Vec<T>) {
    let mut counter = 0;

    while counter < s.len() {
        let mut max = &s[counter];
        let mut max_index = counter;

        for i in counter..s.len() {
            if s[i] > *max {
                max = &s[i];
                max_index = i;
            }
        }

        s.swap(counter, max_index);
        counter += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_examples(
        sort_fn_1: fn(&mut Vec<i32>),
        sort_fn_2: fn(&mut Vec<char>),
        sort_fn_3: fn(&mut Vec<&'static str>),
    ) {
        let mut s1 = vec![5, 10, 4, 3, 20, 12, 9];
        sort_fn_1(&mut s1);
        assert_eq!(vec![20, 12, 10, 9, 5, 4, 3], s1);

        let mut s2 = vec!['a', 'z', 'w', 'v'];
        sort_fn_2(&mut s2);
        assert_eq!(vec!['z', 'w', 'v', 'a'], s2);

        let mut s3 = vec!["foo", "bar", "baz", "dawn", "fam", "dam"];
        sort_fn_3(&mut s3);
        assert_eq!(vec!["foo", "fam", "dawn", "dam", "baz", "bar"], s3);
    }

    #[test]
    fn test_bubble_sort() {
        make_examples(_bubble_sort, _bubble_sort, _bubble_sort);
    }

    #[test]
    fn test_selection_sort() {
        make_examples(_selection_sort, _selection_sort, _selection_sort);
    }
}
