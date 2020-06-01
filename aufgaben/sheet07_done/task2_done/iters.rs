fn factorial(x: u64) -> u64 {
    (1..x+1).product()
}

#[test]
fn test_factorial() {
    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(2), 2);
    assert_eq!(factorial(3), 6);
    assert_eq!(factorial(15), 1_307_674_368_000);
}

fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

#[test]
fn test_is_palindrome() {
    assert!(is_palindrome("bob"));
    assert!(is_palindrome("anna"));
    assert!(is_palindrome("lagerregal"));

    assert!(!is_palindrome("peter"));
}

fn greatest_subsequencial_sum(arr: &[i64]) -> &[i64] {
    // .flat_map(|a| a.into_iter().sum()))
    (1..arr.len() +1)
        // iterate over possible windows
        .flat_map(|l| arr.windows(l))
        // add the empty slice in case all numbers are negative
        .chain(std::iter::once(&arr[0..0]))
        //sum each window and select the maximum
        .max_by_key(|f| f.iter().sum::<i64>())
        //we're sure that there is one returnvalue
        .unwrap()
}

#[test]
fn test_greatest_subsequencial_sum() {
    let a = [1, 2, 39, 34, 20, -20, -16, 35, 0];
    assert_eq!(greatest_subsequencial_sum(&a), &a[0..5]);

    let b = [-3, -9, -8, -34];
    assert_eq!(greatest_subsequencial_sum(&b), &[]);
}

fn rot13(inp: &str) -> String {
    inp.chars().map(|c| 
            match c {
                'a'...'m' | 'A'...'M' => (c as u8 + 13) as char,
                'n'...'z' | 'N'...'Z' => (c as u8 - 13) as char,
                _ => c
            }).collect()
}

#[test]
fn test_rot13() {
    assert_eq!(rot13("hello"), "uryyb");
    assert_eq!(rot13("uryyb"), "hello");

    assert_eq!(
        rot13("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"),
        "NOPQRSTUVWXYZABCDEFGHIJKLMnopqrstuvwxyzabcdefghijklm"
    );

    assert_eq!(rot13("peter"), "crgre");
}

fn used_chars_count(inp: &[&str]) -> usize {
    use std::collections::HashSet;

    inp.iter().
        flat_map(|part| part.chars()).
        filter(|c| !c.is_whitespace()).
        collect::<HashSet<_>>().len()

}

#[test]
fn test_used_letters() {
    assert_eq!(used_chars_count(&["hi", "ih gitt"]), 4);
    assert_eq!(used_chars_count(&["peter"]), 4);
    assert_eq!(used_chars_count(&["p e t e r", "barbara"]), 6);
}

fn main() {}
