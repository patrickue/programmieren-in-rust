//! Task 3.1: Rule 90


fn main() {
    fn print_vector(vec: &[bool]) {
        for &elem in vec {
            if elem { print!("##") } else { print!("  ") }
        }
        println!();
    }

    // TODO: Task 1c)
    let mut vec_old = read_input();
    print_vector(&vec_old); 

    for _ in 0..20 {
        let new_vec = next_step(&vec_old);
        print_vector(&new_vec);

        vec_old = new_vec;
    }

    //println!("Output {:?}", vec); 
}

/// Reads a valid initial configuration for our automaton from the terminal.
fn read_input() -> Vec<bool> {
    // This tries to read a string from the terminal, checks whether it's
    // valid (only contains 1's and 0's). If the user fails to input a correct
    // string, this routine will ask again until the user finally manages to
    // give us a correct string.
    //
    // You don't need to understand this routine yet; that's why I've written
    // it already ;-)
    //
    // You only need to use the `input` variable (of type `String`). You can
    // also assume that it only contains '0' and '1' chars.
    let input = {
        let mut buffer = String::new();

        loop {
            println!("Please give me the initial configuration (a string of '0' and '1'!):");
            buffer.clear();

            // `read_line` returns an error if the input isn't valid UTF8 or if
            // a strange IO error occured. We just panic in that case...
            std::io::stdin()
                .read_line(&mut buffer)
                .expect("something went seriously wrong :O");

            if buffer.trim().chars().all(|c| c == '1' || c == '0') {
                break;
            }
        }

        buffer.trim().to_string()
    };

    let mut vec = Vec::with_capacity(input.len());
    // TODO: Task 1a)
    for c in input.chars() {
        vec.push( c == '1' );
    }
    vec
}

// TODO: Task 1b)
fn next_step(pos: &[bool]) -> Vec<bool>{
    let mut output = vec![false; pos.len()];
    let len = pos.len();
    let mut idx0 = len-1;
    let mut idx2 = 1;
    for i in 0..len {
        output[i] = pos[idx0] ^ pos[idx2];

        //increase indizes
        idx0 += 1;
        idx2 += 1;
        if idx2 >= len { idx2 = 0 };
        if idx0 >= len {idx0 = 0 };
    }
    output 
}

#[test]
fn rule90_rules() {
    assert_eq!(next_step(&[false, false, false]), vec![false, false, false]);
    assert_eq!(next_step(&[ true, false, false]), vec![false,  true,  true]);
    assert_eq!(next_step(&[ true,  true, false]), vec![ true,  true, false]);
    assert_eq!(next_step(&[ true,  true,  true]), vec![false, false, false]);
}
