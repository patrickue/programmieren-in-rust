//! Task 3.2: Pokemon

fn main() {
    let red_pokemon = Pokemon::with_level(choose_pokemon("Red"), 5);
    let blue_pokemon = Pokemon::with_level(choose_pokemon("Blue"), 5);
    println!(">>>>> Status: {} has {} HP, {} has {} HP",
             red_pokemon.name(),
             red_pokemon.stats().hp,
             blue_pokemon.name(),
             blue_pokemon.stats().hp);

    let mut attacker = red_pokemon;
    let mut defender = blue_pokemon;

    loop {
        println!(">>>>> {} is about to attack! Which move shall it execute?", attacker.name());
        print_pokemon_attacks(&attacker);
        let attack_id = read_usize();
        // println!("Attack ID: {}", attack_id);
        let attack = attacker.model().attacks[attack_id];
        defender.endure_attack(&attacker, *attack);
        println!(">>>>> {} uses {}! ({} has {} HP left)", attacker.name(), attack.name, defender.name(), defender.stats().hp);
        if !defender.is_alive() { 
           println!(">>>>> {} fainted!", defender.name());
           break; 
        }

        //Switch roles    
        let tmp = attacker;
        attacker = defender;
        defender = tmp;
    }

    println!("END - no more game");
}

fn choose_pokemon(player: &str) -> &'static PokemonModel {
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

fn find_pokemon_by_name(name_str: &str) -> Option<&'static PokemonModel> {
    for pokemon in POKEDEX {
        if pokemon.name == name_str {
            return Option::Some(pokemon); 
        } 
    }
    Option::None
}

fn print_pokemon_attacks(pokemon: &Pokemon){
    for (idx, attack) in pokemon.model().attacks.iter().enumerate() {
        println!("{}: {}", idx, attack.name);
    }
}

/// The Pokemon as struct to create objects from
///
///
struct Pokemon {
    /// Which model is this Pokemon from?
    model: &'static PokemonModel,
    /// How is the status of this Pokemon
    stats: Stats,
    /// Which level does the Pokemon have
    level: u8,
}

impl Pokemon {
    pub fn with_level(model: &'static PokemonModel, level: u8) -> Self {
        Pokemon {
            model: model,
            stats: Stats::at_level(model.base_stats, level),
            level: level,
        }
    }
    
    pub fn stats(&self) -> Stats {
        self.stats
    }

    pub fn model(&self) -> &'static PokemonModel {
        self.model
    }

    pub fn name(&self) -> &'static str {
        self.model.name
    }

    pub fn level(&self) -> u8 {
        self.level
    }

    pub fn endure_attack(&mut self, attacker: &Pokemon, attack: Attack) {
        self.stats.hp = self.stats.hp.saturating_sub(attack_damage(attacker, self, attack));
    }

    pub fn is_alive(&self) -> bool {
        self.stats.hp > 0
    }
}



/// Describes an attack with all its properties. This type is similar to
/// `PokemonModel`, as there are finite many, immutable instances of this type
/// in a database. This is not a type whose instances change over time.
#[derive(Debug, Clone, Copy)]
struct Attack {
    category: AttackCategory,
    name: &'static str,
    /// Base power of the move. The actual inflicted damage is calculated with
    /// a formula using the move's power and a few other parameters.
    base_power: u8,
    type_: Type,
}

/// Category of an attack.
///
/// Note: currently, the category 'status' is missing.
#[derive(Debug, Clone, Copy)]
enum AttackCategory {
    /// Attacks with body contact, like "Tackle" or "Bite"
    Physical,
    /// Attacks without body contact, like "Bubble Beam" or "Thunderbolt"
    Special,
}

/// Describes how effective an attack of one type is on a Pokemon of another
/// type.
///
/// Note that a Pokemon can have two types. In order to determine the
/// effectiveness, the multipliers of the effectivenesses on both types
/// are multiplied. As such, there can be 0.25 and 4.0 multipliers!
#[derive(Debug, Clone, Copy)]
enum TypeEffectiveness {
    NotEffective,
    NotVeryEffective,
    Normal,
    SuperEffective,
}

impl TypeEffectiveness {
    /// Returns the type effectiveness of an attack from one attacker type
    /// on one defender type.
    fn of_attack(attacker: Type, defender: Type) -> Self {
        use Type::*;
        use TypeEffectiveness as Te;

        // TODO: complete this
        match (attacker, defender) {
            (Fire, Water) => Te::NotVeryEffective,
            (Fire, Grass) => Te::SuperEffective,
            (Water, Fire) => Te::SuperEffective,
            (Water, Grass) => Te::NotVeryEffective,
            (Grass, Fire) => Te::NotVeryEffective,
            (Grass, Water) => Te::SuperEffective,
            _ => Te::Normal,
        }
    }

    /// Returns the corresponding multiplier for the damage formula.
    fn multiplier(&self) -> f64 {
        match *self {
            TypeEffectiveness::NotEffective => 0.0,
            TypeEffectiveness::NotVeryEffective => 0.5,
            TypeEffectiveness::Normal => 1.0,
            TypeEffectiveness::SuperEffective => 2.0,
        }
    }
}


/// Types (sometimes called "elements") of the Pokemon universe. Each
/// attack-move has exactly one type, Pokemons can have one or two types.
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
enum Type {
    Normal,
    Fire,
    Fighting,
    Water,
    Flying,
    Grass,
    Poison,
    Electric,
    Ground,
    Psychic,
    Rock,
    Ice,
    Bug,
    Dragon,
    Ghost,
    Dark,
    Steel,
    Fairy,
}

/// Describes the type of a Pokemon. Pokemon can have one or two types.
#[derive(Debug, Clone, Copy)]
enum PokemonType {
    One(Type),
    Two(Type, Type),
}

/// Describes a kind of Pokemon, e.g. "Pikachu".
///
/// This is different than an actual, living Pokemon. This struct just
/// describes properties that are the same for every creature of this
/// Pokemon kind.
#[derive(Debug, Clone, Copy)]
struct PokemonModel {
    /// Name of the Pokemon
    name: &'static str,
    /// ID in the international Pokedex
    id: u16,
    type_: PokemonType,
    base_stats: Stats,
    /// This is different from the real Pokemon games: attacks are not part
    /// of the Pokemon model, but of the Pokemon itself (as they change over
    /// time). A pokemon just has an abstract learnset of potential attacks.
    /// But this is easier for now.
    attacks: &'static [&'static Attack]
}

/// Describes the basic stats of a Pokemon.
///
/// Each living Pokemon has an actual stat value, but each Pokemon kind also
/// has so called "base stats". These base stats are used to calculate the
/// actual stats, whose depend on the Pokemon's current level. Stronger Pokemon
/// have higher base stats.
#[derive(Debug, Clone, Copy)]
struct Stats {
    /// Health points
    hp: u16,
    /// Speed, sometimes called initiative (INIT)
    speed: u16,
    /// Strength of physical attacks (like "Tackle")
    attack: u16,
    /// Strength of special attacks (like "Bubble Beam")
    special_attack: u16,
    /// Defense power against physical attacks (like "Tackle")
    defense: u16,
    /// Defense power against special attacks (like "Bubble Beam")
    special_defense: u16,
}

impl Stats {
    /// Given the base stats and a level, this function returns the actual
    /// stats for that level.
    ///
    /// This function doesn't implement the correct formula used by Pokemon
    /// games. It is a simplified version of the original formula for now: we
    /// ignore IVs, EVs and the Pokemon's nature). The complete formula can be
    /// found [here (HP)][1] and [here (other stats)][2].
    ///
    /// [1]: http://bulbapedia.bulbagarden.net/wiki/File:HPStatCalcGen34.png
    /// [2]: http://bulbapedia.bulbagarden.net/wiki/File:OtherStatCalcGen34.png
    fn at_level(base: Self, level: u8) -> Self {
        /// The formula is the same for all stats != hp
        fn stat_formula(base: u16, level: u8) -> u16 {
            ((base as f64 * level as f64) / 50.0 + 5.0) as u16
        }

        let hp = (
            (base.hp as f64 * level as f64) / 50.0
                + level as f64
                + 10.0
        ) as u16;

        Stats {
            hp: hp,
            speed: stat_formula(base.speed, level),
            attack: stat_formula(base.attack, level),
            special_attack: stat_formula(base.special_attack, level),
            defense: stat_formula(base.defense, level),
            special_defense: stat_formula(base.special_defense, level),
        }
    }
}

// ===========================================================================
// ===========================================================================
// ===========================================================================
// Formulas to calculate stuff
// ===========================================================================
// ===========================================================================
// ===========================================================================

/// Calculates the damage of an attack. We don't use the exact formula, but
/// a simplified version of it. In particular, we simplified the "Modifier"
/// term quite a bit. The correct and complete formula can be found [here][1].
///
/// [1]: http://bulbapedia.bulbagarden.net/wiki/Damage#Damage_formula
fn attack_damage(attacker: &Pokemon, defender: &Pokemon, attack: Attack) -> u16 {
    // Depending on the attack category, get the correct stats
    let (attack_mod, defense_mod) = match attack.category {
        AttackCategory::Physical => {
            (attacker.stats().attack, defender.stats().defense)
        }
        AttackCategory::Special => {
            (attacker.stats().special_attack, defender.stats().special_defense)
        }
    };

    // Cast everything to f64 to reduce noise in actual formula
    let (attack_mod, defense_mod) = (attack_mod as f64, defense_mod as f64);
    let base_power = attack.base_power as f64;
    let attacker_level = attacker.level() as f64;

    // The modifier only depends on the type effectiveness (in our simplified
    // version!).
    let modifier = match defender.model().type_ {
        PokemonType::One(ty) => {
            TypeEffectiveness::of_attack(attack.type_, ty).multiplier()
        }
        PokemonType::Two(ty_a, ty_b) => {
            TypeEffectiveness::of_attack(attack.type_, ty_a).multiplier()
                * TypeEffectiveness::of_attack(attack.type_, ty_b).multiplier()
        }
    };

    // With every parameter prepared above, here is the formula
    (
        (
            ((2.0 * attacker_level + 10.0) / 250.0)
                * (attack_mod / defense_mod)
                * base_power
                + 2.0
        ) * modifier
    ) as u16
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
