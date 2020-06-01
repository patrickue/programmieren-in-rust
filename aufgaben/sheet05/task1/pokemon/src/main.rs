
mod db;
mod engine;
mod game;

use game::{fight, choose_pokemon};
use engine::Pokemon;

fn main() {
    let red_pokemon = Pokemon::with_level(choose_pokemon("Red"), 5);
    let blue_pokemon = Pokemon::with_level(choose_pokemon("Blue"), 5);
    println!(">>>>> Status: {} has {} HP, {} has {} HP",
             red_pokemon.name(),
             red_pokemon.stats().hp,
             blue_pokemon.name(),
             blue_pokemon.stats().hp);

    fight(red_pokemon, blue_pokemon);

    println!("END - no more game");
} 
