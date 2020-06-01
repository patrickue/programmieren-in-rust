
extern crate term_painter;

use std;

use self::term_painter::{ToStyle, Color};
use self::term_painter::Color::{Blue, Red, Yellow, Green};
//use term_painter::Attr::*;

use db::find_pokemon_by_name;
use db::types::PokemonModel;
use db::data::POKEDEX;
use engine::Pokemon;

pub fn fight(red_pokemon: Pokemon, blue_pokemon: Pokemon){
    let mut attacker = red_pokemon;
    let mut defender = blue_pokemon;
    let mut att_color = Red;
    let mut def_color = Blue;

    loop {
        println!(">>>>> {} is about to attack! Which move shall it execute?", att_color.paint(attacker.name()));
        print_pokemon_attacks(&attacker);
        let attack_id = read_usize();
        // println!("Attack ID: {}", attack_id);
        let attack = attacker.model().attacks[attack_id];
        defender.endure_attack(&attacker, *attack);
        println!(">>>>> {} uses {}! ({} has {} HP left)", att_color.paint(attacker.name()), Yellow.paint(attack.name), def_color.paint(defender.name()), Green.paint(defender.stats().hp));
        if !defender.is_alive() {
           println!(">>>>> {} fainted!", def_color.paint(defender.name()));
           break;
        }

        //Switch roles
        let tmp = attacker;
        attacker = defender;
        defender = tmp;

        let tmp_color = att_color;
        att_color = def_color;
        def_color = tmp_color;
    }
}

pub fn choose_pokemon(player: &str) -> &'static PokemonModel {
    loop {
        println!("Player {}, please choose a Pokemon (or type '?' to get a complete list)",
        player);

        let input_str = read_string();
        if input_str == "?" {
            print_pokemon_list();
        } else {
            match find_pokemon_by_name(&input_str) {
                None => {},
                Some(pkm_model) => {
                    return pkm_model;
                }
            }
        }
    }
}

fn print_pokemon_list() {
    let mut i = 1;
    for elem in POKEDEX {
        println!("#{:03} {}", i, elem.name);
        i += 1;
    }
}

fn print_pokemon_attacks(pokemon: &Pokemon){
    for (idx, attack) in pokemon.model().attacks.iter().enumerate() {
        println!("{}: {}", idx, attack.name);
    }
}


// ===========================================================================
// ===========================================================================
// ===========================================================================
// ===========================================================================
// Helper functions (you don't need to understand how they work yet)
// ===========================================================================
// ===========================================================================
// ===========================================================================

/// Reads a string from the terminal/user.
fn read_string() -> String {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("something went horribly wrong...");

    // Discard trailing newline
    let new_len = buffer.trim_right().len();
    buffer.truncate(new_len);

    buffer
}

/// Reads a valid `usize` integer from the terminal/user.
fn read_usize() -> usize {
    loop {
        match read_string().parse() {
            Ok(res) => return res,
            Err(_) => println!("That was not an integer! Please try again!"),
        }
    }
}


