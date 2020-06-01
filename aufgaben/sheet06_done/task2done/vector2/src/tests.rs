#[cfg(test)]
mod tests {
    use Vector2;

    #[test]
    fn new_vector() {
        let v_should = Vector2{x: 1, y: 3};
        assert_eq!(v_should, Vector2::new(1, 3));
    }

    #[test]
    fn test_origin_constr() {
        let v_org = Vector2{x: 0, y: 0};
        assert_eq!(v_org, Vector2::origin());
    }

    #[test]
    fn test_unit_x_constr() {
        let v_org = Vector2{x: 1, y: 0};
        assert_eq!(v_org, Vector2::unit_x());
    }

    #[test]
    fn test_unit_y_constr() {
        let v_org = Vector2{x: 0, y: 1};
        assert_eq!(v_org, Vector2::unit_y());
    }

    #[test]
    fn test_add() {
        let v_a = Vector2{x: 7, y: 1};
        let v_b = Vector2{x: 4, y: 22};
        let v_sum = Vector2{x: 11, y: 23};
        assert_eq!(v_sum, v_a+v_b);
    }

    #[test]
    fn test_mul() {
        let v_a = Vector2{x: 7, y: 1};
        let v_b = 4;
        let v_mult = Vector2{x: 28, y: 4};
        assert_eq!(v_mult, v_a*v_b);
    }

}
