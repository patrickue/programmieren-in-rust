use std::fmt;

#[derive(PartialEq, Debug)]
struct Swagger<T>(pub T);


impl <T: fmt::Display> fmt::Display for Swagger<T> 
    where T: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "yolo {} swag", self.0)
    }
}

trait SwaggerExt: Sized {
    fn with_swag(self) -> Swagger<Self>;
}

impl<T> SwaggerExt for T {
    fn with_swag(self) -> Swagger<Self> {
       Swagger(self) 
    }
}


fn main () {
    let swag = Swagger(3);
    let seven = 7;
    println!("With swag: {}", swag);
    println!("With swag: {}", seven.with_swag());
}
