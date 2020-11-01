#[macro_use]
extern crate clap;

use clap::{App, Arg};
use std::error::Error;

mod player;
mod playfield;

use player::{Player, PlayerBehaviour};
use crate::player::{PLAYER_TYPES, new_player_with_type};
use playfield::PlayField;

#[cfg(test)]
mod tests;

/// Does most of the work: parses the command line args and executes the
/// subcommand.
fn parse_and_exec() -> Result<(), Box<dyn Error>> {

    let matches = App::new("TicTacToe")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Offers several tools related to randomness")
        .arg(
            Arg::with_name("Player1")
                //.about("What type of player is Player1")
                .index(1)
                .possible_values(PLAYER_TYPES)
                .required(true),
        )
        .arg(
            Arg::with_name("Player2")
                //.about("What type of player is Player1")
                .index(2)
                .possible_values(PLAYER_TYPES)
                .required(true),
        )
        .get_matches();

    // Note, it's safe to call unwrap() because the arg is required
    let player1 = new_player_with_type(
        1,
        matches.value_of("Player1").unwrap(),
        playfield::Square::Cross);
    let player2 = new_player_with_type(
        2,
        matches.value_of("Player2").unwrap(),
        playfield::Square::Circle);

    let_the_games_begin(player1, player2);

    Ok(())
}

fn let_the_games_begin<T: ?Sized>(player1: Box<T>, player2: Box<T>)
    where T: PlayerBehaviour + std::fmt::Display
{
    println!("{}", player1);
    println!("{}", player2);

    let mut playfield = PlayField::new_empty();
    println!("Playfield {}", playfield);

    loop {
        player1.choose_and_mark_square(&mut playfield);
        println!("Playfield {}", playfield);
        match playfield.check_if_somebody_won() {
            Some(x) => {
                println!("Player 1 with {} won!", x);
                break;
            }
            None => {}
        }
        player2.choose_and_mark_square(&mut playfield);
        println!("Playfield {}", playfield);
        match playfield.check_if_somebody_won() {
            Some(x) => {
                println!("Player 2 with {} won!", x);
                break;
            }
            None => {}
        }
    }
}


fn main() {
    let res = parse_and_exec();
    if let Err(e) = res {
        println!("{}", e);
    }
}

// ===========================================================================
// Helper functions
// ===========================================================================

/// Reads a string from the terminal/user.
fn read_string() -> String {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("something went horribly wrong...");

    // Discard trailing newline
    let new_len = buffer.trim_end().len();
    buffer.truncate(new_len);

    buffer
}