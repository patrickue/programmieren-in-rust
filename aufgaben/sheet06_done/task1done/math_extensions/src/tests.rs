
#[cfg(test)]
mod tests {
    use math_extensions::{clamp, sum_product};
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_clamp_with_usize() {
        assert_eq!(clamp(3, 5, 10), 5);
        assert_eq!(clamp(112, 5, 10), 10);
        assert_eq!(clamp(-3, 5, 10), 5);
        assert_eq!(clamp(-1, -7, -4), -4);
    }
    
    #[test]
    fn test_sum_product_with_usize() {
        assert_eq!(sum_product(3, 5), (8, 15));
        assert_eq!(sum_product(3, 5), (8, 15));
    }
    
    use math_extensions::BoolExt;
    #[test]
    fn test_bool_to_option() {
        assert_eq!(Some(3), true.into_option(3));
        assert_eq!(None, false.into_option("susi"));
    }
}
