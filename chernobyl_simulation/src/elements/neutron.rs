use macroquad::prelude::*;
use ::rand::Rng;

const PITAGYROS: f32 = 2.0;
const VELOCITY_MULTIPLYER: f32 = 2.5;
const NEUTRON_RADIUS: f32 = 5.0;

pub struct Neutron {
    pub position: Vec2,
    pub direction: Vec2,
    pub is_thermal: bool,
}
impl Neutron {
    pub fn new_initial() -> Neutron {
        let position: Vec2 = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);
        let direction: Vec2 = new_random_direction();
        let is_thermal: bool = false;
        Neutron { position, direction, is_thermal }
    }
    pub fn new_from_reaction(x: f32, y: f32) -> Neutron {
        let position: Vec2 = Vec2::new(x, y);
        let direction: Vec2 = new_random_direction();
        let is_thermal: bool = false;
        Neutron { position, direction, is_thermal }
    }
    pub fn draw(&self) {
        let mut color: Color = RED;
        if self.is_thermal { color = WHITE }
        draw_circle(self.position.x, self.position.y, NEUTRON_RADIUS, color);
    }
    pub fn run(&mut self) {
        self.position.x += self.direction.x;
        self.position.y += self.direction.y;
    }
    pub fn make_thermal(&mut self) {
        if !self.is_thermal {
            self.direction.x /= VELOCITY_MULTIPLYER;
            self.direction.y /= VELOCITY_MULTIPLYER;
            self.is_thermal = true;
        }
    }
}
fn new_random_direction() -> Vec2 {
    let mut rng: ::rand::prelude::ThreadRng = ::rand::thread_rng();
    let mut dx: f32 = rng.gen_range(-PITAGYROS..=PITAGYROS);
    let mut dy: f32 = ((PITAGYROS * PITAGYROS) - (dx * dx)).sqrt();
    let is_lower: bool = rng.gen_bool(0.5);
    if is_lower { dy = -dy }
    dx *= VELOCITY_MULTIPLYER;
    dy *= VELOCITY_MULTIPLYER;
    Vec2::new(dx, dy)
}