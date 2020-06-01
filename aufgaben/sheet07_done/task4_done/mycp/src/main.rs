use std::env;
use std::fs::File as File;
use std::io::Error as Error;
use std::io::ErrorKind as ErrorKind;
use std::ffi::OsString;

// Error type for what can go wrong on parsing arguments for this cmd
#[derive(Debug)]
enum ArgsError {
    NotEnoughArgs,
    TooManyArgs(usize),
    NotUtf8(OsString),
}

fn get_args() -> Result<(String, String), ArgsError>{
    // Prints each argument on a separate line
    let mut ret_args: Vec<String> = Vec::new();
    let args: Vec<_> = env::args().collect();
    
    match env::args_os().count() {
        n if n > 3 => return Err(ArgsError::TooManyArgs(n - 1)), 
        n if n < 3 => return Err(ArgsError::NotEnoughArgs),
        _ => {}
    }
    
    env::args_os()
        //get me the first two
        .skip(1)
        .take(2)
        //map OsString into utf8
        .map( |oss| oss.into_string())
        // collect to get the Results on the outside
        .collect::<Result<Vec<_>, _>>()
        //convert vector into tuple of Strings
        .map(|mut v| (v.remove(0), v.remove(0)))
        //wrap conversion error into our Error
        .map_err(|oss| ArgsError::NotUtf8(oss))
}

fn copy(inputname: String, outputname: String) -> Result<(), std::io::Error>{
    use std::io::BufRead;
    use std::io::BufReader as BufReader;
    use std::io::LineWriter as LineWriter;
    use std::io;

    // TODO: Add some details that then inputfile
    // caused a problem.
    let mut fi = File::open(inputname)?;

    let mut fo = File::create(outputname)?;

    io::copy(&mut fi, &mut fo)
        //we are not interested in the number of bytes written
        .map(|_| ())
}

fn main() {
    let (arg1, arg2) = match get_args() {
        Ok(a) => a,
        Err(text) => { 
            println!("{:?} Usage: mycp <source> <target>", text);
            std::process::exit(1);
        },
    };
    println!("Args: {:?}, {:?}", arg1, arg2);
    let res = copy(arg1, arg2);
    println!("Result from copy: {:?}", res);
}
