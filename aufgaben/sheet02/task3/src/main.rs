fn main() {
    println!("Hello, world!");
    println!("Counted {}", count("peddter, Hello, Peter, é", 'e'));
}

fn count(string: &str, find_char: char) -> u32 {
    let mut cnt = 0;
    for c in string.chars() { 
        if c == find_char {
            cnt+=1;
        }
    }
    cnt
}
