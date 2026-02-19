use crate::elements::neutron::*;
use macroquad::prelude::*;
use ::rand::prelude::*;
pub struct Waste {
    pub waste_elements: Circle,
}
impl Waste {
    pub fn new(waste_position: Vec2, waste_radius: f32) -> Waste {
        let waste_elements: Circle = Circle::new(waste_position.x, waste_position.y, waste_radius);
        Waste { waste_elements }
    }
    pub fn spontaneous_neutron_throw(&self, probability: f64, total_velocity: f32) -> Option<Neutron> {
        let mut rng: ThreadRng = ::rand::thread_rng();
        let is_throwed: bool = rng.gen_bool(probability);
        if is_throwed {
            let waste_position: Vec2 = Vec2::new(self.waste_elements.x, self.waste_elements.y);
            let waste_radius: f32 = self.waste_elements.r;
            return Some(Neutron::new_with_move(waste_position, 
            waste_radius, total_velocity));
        }
        None
    }
    pub fn try_turn_to_uranium(&self, probability: f64) -> bool {
        let mut rng: ThreadRng = ::rand::thread_rng();
        let is_turned: bool = rng.gen_bool(probability);
        is_turned
    }
    pub fn draw(&self) {
        draw_circle(self.waste_elements.x, self.waste_elements.y, self.waste_elements.r, LIGHTGRAY);
    }
}