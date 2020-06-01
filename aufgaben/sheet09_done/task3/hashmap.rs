use std::collections::HashMap;

#[macro_export]
macro_rules! hash_map {
    ( $( $x:expr => $y:expr ),* ) => {
        { // make this a block-expr
            let mut hm = HashMap::new();
            $( hm.insert($x, $y); )*
            hm
        }
    };
}

fn main() {
    let ages = hash_map!{ "Sabine" => 26, "Peter" => 32 };
    println!("{:#?}", ages);
}
