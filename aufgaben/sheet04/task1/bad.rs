/// print if they are both
fn main() {
    for i in 1..21 {
        if is_happy_prime(i) {
            println!("{} is a happy prime!", i);
        }
    }
}

/// Check if a given number is prime and "happy number"
fn is_happy_prime(n: usize) -> bool {
    is_happy(n) && is_prime(n)
}

/// Is it a happy number? https://en.wikipedia.org/wiki/Happy_number
fn is_happy(mut n: usize) -> bool {
    
    fn sum_of_square_of_digits(mut m: usize) -> usize {
        let mut tmp = 0;
        while m > 0 {
            tmp = tmp + (m %10) * (m %10);
            m = m / 10;
        }
        return tmp;
    }

    while n > 1 {
        n = sum_of_square_of_digits(n);
        // We ended up in a cycle -> not happy
        if n == 4 {
            return false;
        }
    }

    return true;
}

/// is it priem?
fn is_prime(n: usize) -> bool {
    // 0 and 1 are automatically no primes
    if n < 2 {
        return false;
    }

    for divisor in 2..(n/2) {
        if n % divisor == 0 {
            return false;
        }
    }

    return true;
}

#[test]
fn test_is_prime() {
    assert_eq!(is_prime(1), false);
    assert_eq!(is_prime(2), true);
    assert_eq!(is_prime(3), true);
    assert_eq!(is_prime(5), true);
    assert_eq!(is_prime(7), true);
    assert_eq!(is_prime(9), false);
    assert_eq!(is_prime(11), true);
    assert_eq!(is_prime(13), true);
    assert_eq!(is_prime(23), true);
    assert_eq!(is_prime(24), false);
}

#[test]
fn test_is_happy() {
    let happy_arr = [1, 7, 10, 13, 19, 23, 28, 31, 32, 338, 356, 874, 881, 1000];
    let sad_arr = [2, 3, 14, 25, 341, 880, 999];

    for &i in happy_arr.iter() {
        assert_eq!(is_happy(i), true);
    }
    for &i in sad_arr.iter() {
        assert_eq!(is_happy(i), false);
    }
}
