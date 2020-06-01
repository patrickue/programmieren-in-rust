fn main() {
    let x = 8;
    println!("Fibonacci number #{}: {}", x, fibonacci(x));
    
    for fib in Fib::new().take(20) {
        println!("Other num: {}", fib);
    }
}

fn fibonacci(i: u64) -> usize {
    let mut prev = 0;
    let mut now = 1;
    for _ in 1..i {
        let mut new = prev + now;
        prev = now;
        now = new;
    }
    now
}

struct Fib {
    prev: u64,
    num: u64,
}

impl Fib {
    fn new () -> Self {
        Fib { 
            num: 1, 
            prev: 0 
        }
    }
}

impl Iterator for Fib {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let tmp = self.prev;
        self.prev = self.num;
        self.num += tmp;
        Some(self.num)
    }
}
