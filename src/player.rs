use std::fmt::Display;

pub struct Player {
    pub health: u32,
    pub score: u32,
    pub bytes: u32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            health: 100,
            score: 0,
            bytes: 0,
        }
    }

    /// returns true if the player dies from the damage
    pub fn damage(&mut self, amount: u32) -> bool {
        if self.health > amount {
            self.health -= amount;
            false
        } else {
            self.health = 0;
            true
        }
    }

    pub fn heal(&mut self, amount: u32) {
        self.health += amount;
    }
}

impl Default for Player {
    fn default() -> Self { Self::new() }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Health: {}", self.health)?;
        writeln!(f, "Score: {}", self.score)?;
        write!(f, "Bytes: {}", self.bytes)?;
        Ok(())
    }
}