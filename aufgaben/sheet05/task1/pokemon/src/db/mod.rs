pub mod data;
pub mod types;

use db::types::PokemonModel;
use db::data::POKEDEX;

pub fn find_pokemon_by_name(name_str: &str) -> Option<&'static PokemonModel> {
    for pokemon in POKEDEX {
        if pokemon.name == name_str {
            return Option::Some(pokemon);
        }
    }
    Option::None
}
