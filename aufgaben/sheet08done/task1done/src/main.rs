
pub trait Times {
    fn times<F>(&self, mut _f: F) -> () 
        where F: FnMut(usize) -> () {
    }
}

impl Times for usize{
    fn times<F>(&self, mut f: F) -> () 
        where F: FnMut(usize) -> () {
        (0..(*self)).for_each(|i| f(i));
        /*for i in (1..(*self+1)).rev(){
            f(i);
        }*/
    }
}

fn main() {
    3.times(|i| {
        println!("Ferris ate {} cookies", i);
    });
}
