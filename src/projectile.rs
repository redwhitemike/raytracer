use crate::tuple::Tuple;
use std::fmt::{Display, Formatter};

pub struct Projectile {
    pub position: Tuple,
    pub velocity: Tuple,
}

impl Display for Projectile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "position: {} - velocity: {}",
            self.position, self.velocity
        )
    }
}

#[derive(Debug, Clone)]
pub struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

impl Projectile {
    // create a new projectile struct
    pub fn new(position: Tuple, velocity: Tuple) -> Self {
        Self { position, velocity }
    }

    // calculate the new vector projectile position based on
    // the given environment
    pub fn tick(&mut self, environment: Environment) {
        self.position = self.position.clone() + self.velocity.clone();
        self.velocity = self.velocity.clone() + environment.gravity + environment.wind;
    }
}

impl Environment {
    // create a new environment struct
    pub fn new(gravity: Tuple, wind: Tuple) -> Self {
        Self { gravity, wind }
    }
}
