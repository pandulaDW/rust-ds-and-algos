use std::collections::HashMap;

fn _factorial(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return 1;
    }

    return n * _factorial(n - 1);
}

fn _fibonacci(n: u32) -> u32 {
    let mut cache: HashMap<u32, u32> = HashMap::with_capacity((n + 1) as usize);
    cache.insert(0, 0);
    cache.insert(1, 1);

    fn fib(n: u32, cache: &mut HashMap<u32, u32>) -> u32 {
        if let Some(cached_val) = cache.get(&n) {
            return *cached_val;
        }
        let fib_value = fib(n - 1, cache) + fib(n - 2, cache);
        cache.insert(n, fib_value);
        return fib_value;
    }

    fib(n, &mut cache)
}

fn _power(n: u64, m: u64) -> u64 {
    if n == 0 {
        return 1;
    }

    return m * _power(n - 1, m);
}

#[cfg(test)]
mod tests {
    use crate::recursion::easy::*;

    #[test]
    fn test_factorial() {
        assert_eq!(1, _factorial(0));
        assert_eq!(1, _factorial(1));
        assert_eq!(24, _factorial(4));
        assert_eq!(120, _factorial(5));
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(0, _fibonacci(0));
        assert_eq!(1, _fibonacci(1));
        assert_eq!(1, _fibonacci(2));
        assert_eq!(2, _fibonacci(3));
        assert_eq!(5, _fibonacci(5));
        assert_eq!(21, _fibonacci(8));
    }

    #[test]
    fn test_power() {
        assert_eq!(2, _power(1, 2));
        assert_eq!(1, _power(0, 7));
        assert_eq!(32, _power(5, 2));
        assert_eq!(3_u64.pow(5), _power(5, 3));
        assert_eq!(4_u64.pow(20), _power(20, 4));
    }
}
