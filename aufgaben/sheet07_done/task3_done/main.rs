use std::iter::Product;

/*
trait FoldProduct<T>: Iterator<Item = T> {
    fn fold_as_product<P>(self) -> P where
        P: Product;
}

impl<T> FoldProduct<T> for T where T: std::iter::Iterator{
    fn fold_as_product<P>(self) -> P where
        P: Product
    {
    }

}
*/

fn fold_as_product(arr: &[i64]) -> i64 {
    arr.into_iter().fold(1, |a, b| a*b)
}

fn main(){}

#[test]
fn test_product() {
    let a = [3, 9, 8, -1];
    let b = [-3, -9, -8, -34];

    assert_eq!(fold_as_product(&a), -216); 
    assert_eq!(fold_as_product(&b), b.into_iter().product()); 
}

fn fold_as_max(arr: &[i64]) -> Option<&i64> {
    arr.into_iter().fold(None, |max, a| 
        match max {
            Some(max) => {if a > max {Some(a)} else{ Some(max)}},
            None => Some(a),
        }
    )
}

#[test]
fn test_max() {
    let a = [3, 9, 8, -1];
    let b = [-3, -9, -8, -34];
    let res = 9;
    assert_eq!(fold_as_max(&a), Some(&res)); 
    assert_eq!(fold_as_max(&b), b.into_iter().max()); 
}

fn fold_as_all<F>(arr: &[bool], f: F) -> bool 
    where F: Fn(&bool) -> bool {
    arr.into_iter().fold(true, |res, a| res & f(a))
}

#[test]
fn test_all() {
    let a = [true, true, true, true];
    let b = [true, true, false, true, true];

    print!("{:?}", a.into_iter().all(|&x| x==true));
    print!("{:?}", b.into_iter().all(|&x| x==true));
    assert_eq!(fold_as_all(&a, |&x| x==true), a.into_iter().all(|&x| x==true));
    assert_eq!(fold_as_all(&b, |&x| x==true), b.into_iter().all(|&x| x==true));
}
