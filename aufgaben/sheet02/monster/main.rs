struct Monster{
    health: u8,
    strength: u8,
}

impl Monster{
    //constructor
    fn new() -> Self {
        Monster { health: 100, strength: 100 }
    }

    fn with_strength(strength: u8) -> Self {
        Monster { health: 100, strength: strength }
    }

    fn weak() -> Self {
        Self::with_strength(10)
    }

    fn is_alive(&self) -> bool {
        self.health > 0
    }

    fn attack_strength(&self) -> u8 {
        if self.health >= 20 {
            return self.strength;
        } else {
            return self.strength / 2;
        }
    }

    fn endure_attack(&mut self, attack_strength: u8) {
        self.health = self.health.saturating_sub(attack_strength);
    }
}
