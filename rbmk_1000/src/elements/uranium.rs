use crate::elements::neutron::*;
use macroquad::prelude::*;
use ::rand::prelude::*;
pub struct Uranium {
    pub uranium_element: Circle,
}
impl Uranium {
    pub fn new(uranium_position: Vec2, uranium_radius: f32) -> Uranium {
        let uranium_element: Circle = Circle::new(uranium_position.x, uranium_position.y, uranium_radius);
        Uranium { uranium_element }
    }
    pub fn try_fission(&self, neutron: &Neutron, total_velocity: f32, neutron_multiplyer: usize)
    -> Option<Vec<Neutron>> {
        if neutron.is_thermal() && self.uranium_element.contains(&neutron.position) {
            let uranium_position: Vec2 = Vec2::new(self.uranium_element.x, self.uranium_element.y);
            let uranium_radius: f32 = self.uranium_element.r;
            let mut new_neutron_vec: Vec<Neutron> = Vec::new();
            for _ in 0..neutron_multiplyer {
                new_neutron_vec.push(Neutron::new_with_move(uranium_position, uranium_radius, total_velocity));
            }            
            return Some(new_neutron_vec);
        }
        None
    }
    pub fn try_turn_to_xenon(&self, probability: f64) -> bool {
        let mut rng: ThreadRng = ::rand::thread_rng();
        let is_turned: bool = rng.gen_bool(probability);
        is_turned
    }
    pub fn draw(&self) {
        draw_circle(self.uranium_element.x, self.uranium_element.y, self.uranium_element.r, BLUE);
    }
}