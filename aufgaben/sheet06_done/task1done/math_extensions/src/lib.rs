mod tests;

mod math_extensions {
    use std::ops::{Add, Mul};

    pub fn clamp<T>(input: T, min: T, max: T) -> T 
        where T: PartialOrd 
    {
        match() {
            () if input < min => min,
            () if input > max => max, 
            _ => input,
        }
    }

    pub fn sum_product<T, U>(a: T, b: U) 
        -> (<T as Add<U>>::Output, <T as Mul<U>>::Output) 
        where T: Mul<U> + Add<U> + Clone,
              U: Clone {
        let sum = a.clone()+b.clone();
        let prod = a*b;
        return (sum, prod); 
    }
    
    use std::option::{Option};

    pub trait BoolExt {
        fn into_option<T>(self, value: T) -> Option<T>;
    }

    impl BoolExt for bool {
        fn into_option<T>(self, value: T) -> Option<T> {
            match self {
                true => Some(value),
                false => None,
            }
        }
    }

}
