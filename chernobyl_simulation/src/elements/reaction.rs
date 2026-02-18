use macroquad::prelude::*;
pub enum Reaction {
    Fission(Vec2),
    Poison,
    None,
}