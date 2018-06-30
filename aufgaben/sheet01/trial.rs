fn greet(mut name: String) -> String{
    println!("Hi {}!", name);
    name = "Susi".to_string();
    name
}

fn main () {
    let peter = "Peter".to_string();
    let mut susi = greet(peter);
    greet(susi);
}
