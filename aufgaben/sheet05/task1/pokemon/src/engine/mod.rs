pub mod canon;

use db::types::{Attack, PokemonModel}; 
use engine::canon::attack_damage;

/// The Pokemon as struct to create objects from
///
///
pub struct Pokemon {
    /// Which model is this Pokemon from?
    model: &'static PokemonModel,
    /// How is the status of this Pokemon
    stats: Stats,
    /// Which level does the Pokemon have
    level: u8,
}

use db::types::Stats;

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

