//! Aufgabe 2.1: Primzahltest

fn main() {
    let array = [3, 5, 6, 7, 9, 11, 15, 32, 33, 34, 35];
    for elem in array.iter() {
        println!("Is {} a prime? {}", elem, is_prime(*elem))
    }
}

fn is_prime(num: i32) -> bool {
    if num >= 3 {
        for i in 2..num {
            if num%i == 0 {
                return false
            }
        }
    }
    true
}

#[test]
fn small_primes() {
    assert!(is_prime(2));
    assert!(is_prime(3));
    assert!(is_prime(5));
    assert!(is_prime(7));
}

#[test]
fn small_composites() {
    assert!(!is_prime(1));
    assert!(!is_prime(4));
    assert!(!is_prime(6));
    assert!(!is_prime(8));
    assert!(!is_prime(9));
}

#[test]
fn large_primes() {
    assert!(is_prime(1_300_769));
    assert!(is_prime(1_300_297));
    assert!(is_prime(7_367_287));
}

#[test]
fn large_composites() {
    assert!(!is_prime(908_209));
    assert!(!is_prime(3_073_009));
    assert!(!is_prime(4_897_369));
}
