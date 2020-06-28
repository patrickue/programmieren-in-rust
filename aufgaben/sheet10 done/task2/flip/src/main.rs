use clap::{App,Arg, SubCommand};
use rand::Rng;
use rand::seq::SliceRandom;
//use rand::prelude::*;

#[derive(Debug)]
struct CustomError(String);

#[cfg(test)]
mod tests;

fn coin() -> String {
    // generates a boolean
    if rand::random() {
        "heads".to_string()
    } else {
        "tails".to_string()
    }
}

fn dice(sides: usize) -> usize {
    let mut rng = rand::thread_rng();
    // simulate rolling a die:
    rng.gen_range(1, sides+1)
}

fn choose(mut list: Vec<String>, count: usize) -> Vec<String> {
    let mut rng = rand::thread_rng();
    list.shuffle(&mut rng);
    list[0..count].to_vec()
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let matches = App::new("MyApp")
        // Normal App and Arg configuration goes here...
        // In the following example assume we wanted an application which
        // supported an "add" subcommand, this "add" subcommand also took
        // one positional argument of a file to add:
        .arg(Arg::with_name("times")
               .short("t")
               .long("times")
               .takes_value(true)
               .help("'A number of executions you want to have on the choosen method'")
        )
        .subcommand(
            SubCommand::with_name("coin") // The name we call argument with
                .about("Throw a coin") // The message displayed in "myapp -h"
                .version("0.1") // Subcommands can have independent version
                .author("PT") // And authors
        )
        .subcommand(
            SubCommand::with_name("dice") // The name we call argument with
                .arg(Arg::with_name("sides")
                     .short("s")
                     .long("sides")
                     .value_name("SIDES")
                     .help("Sets a number of sides for the dice to have.")
                     .takes_value(true))
                .about("Roll a dice") 
                .version("0.1") 
                .author("PT") 
        )
        .subcommand(
            SubCommand::with_name("choose") 
                .arg(Arg::with_name("count")
                     .short("c")
                     .long("count")
                     .value_name("COUNT")
                     .help("Sets a number of options to choose randomly from the sequence")
                     .takes_value(true))
                .arg(Arg::with_name("sequence")
                     .multiple(true)
                     .min_values(2)
                     .takes_value(true))
                .about("Choose from a list") 
                .version("0.1") 
                .author("PT") 
        )
        .get_matches();

    let times_str = matches.value_of("times");

    let times_res = times_str.unwrap_or("1").parse::<i32>();
    let times = match times_res {
        Ok(n) => n,
        Err(_n) => {
            eprintln!("No valid number for --times.");
            std::process::exit(1);
        } 
    };

    println!("We will run this {} times", times);
    
    //Check for the subcommands
    if matches.is_present("coin") {
        //println!("'flip coin' was run.");
        for _ in 0..times {
            println!("{}", coin());
        }
        Ok(())
    }
    else if let Some(sub_matches) = matches.subcommand_matches("dice") {
        let sides_str = sub_matches.value_of("sides");
        println!("Value for sides: {:?}", sides_str);
        
        match sides_str.unwrap_or("6").parse::<usize>() {
            Ok(n) => {
                for _ in 0..times {
                    println!("{}", dice(n));
                }
                Ok(())
            }
            Err(_) => {
                eprintln!("Not a number for sides!");
                std::process::exit(1);
            }
        }
        
    }
    else if let Some(sub_matches) = matches.subcommand_matches("choose") {
        let count_str = sub_matches.value_of("count");
        println!("Value for count: {:?}", count_str);

        let count = match count_str.unwrap_or("1").parse::<usize>() {
            Ok(n) => n,
            Err(_) => 
            {
                eprintln!("Not a number for count!");
                std::process::exit(1);
            }
        };

        let mut sequence: Vec<String>; 
        if sub_matches.is_present("sequence") {
            let iterator = sub_matches.values_of("sequence");
            sequence = iterator.unwrap().map(|s| s.to_string()).collect()
        }
        else
        {
            return Err(Box::new(error::Error {message:"Specify at least two options to choose from.".to_string()}));
            //std::process::exit(1);
        }
        if sequence.len() < count {
            eprintln!("Count cannot be higher than number of elements.");
            std::process::exit(1);
        }
        else {
            println!("{:?}", choose(sequence, count));
            Ok(())
        }
    }
    else
    {
        panic!("Not to be reached");
    }
}
