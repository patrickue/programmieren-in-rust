
fn main()
{
    let start_val = 27;
    let mut count_val = start_val;

    println!();

    while count_val != 1
    {
        count_val = if count_val % 2 == 0 { //todo, use .is_even()
            count_val / 2
        } else {
            3*count_val + 1
        };
        println!("{}", count_val)
    }
    println!("Hello World!");
}

