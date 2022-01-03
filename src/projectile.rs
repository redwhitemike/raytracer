use crate::tuple::Tuple;
use num::Float;
use std::fmt::{Display, Formatter};

pub struct Projectile<T>
where
    T: Float,
{
    pub position: Tuple<T>,
    pub velocity: Tuple<T>,
}

impl<T> Display for Projectile<T>
where
    T: Float,
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "position: {} - velocity: {}",
            self.position, self.velocity
        )
    }
}

#[derive(Debug, Clone)]
pub struct Environment<T>
where
    T: Float,
{
    gravity: Tuple<T>,
    wind: Tuple<T>,
}

impl<T> Projectile<T>
where
    T: Float,
{
    // create a new projectile struct
    pub fn new(position: Tuple<T>, velocity: Tuple<T>) -> Self {
        Self { position, velocity }
    }

    // calculate the new vector projectile position based on
    // the given environment
    pub fn tick(&mut self, environment: Environment<T>) {
        self.position = self.position.clone() + self.velocity.clone();
        self.velocity = self.velocity.clone() + environment.gravity + environment.wind;
    }
}

impl<T> Environment<T>
where
    T: Float,
{
    // create a new environment struct
    pub fn new(gravity: Tuple<T>, wind: Tuple<T>) -> Self {
        Self { gravity, wind }
    }
}
