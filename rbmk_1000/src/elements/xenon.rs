use crate::elements::neutron::*;
use macroquad::prelude::*;
pub struct Xenon {
    pub xenon_elements: Circle,
}
impl Xenon {
    pub fn new(xenon_position: Vec2, xenon_radius: f32) -> Xenon {
        let xenon_elements: Circle = Circle::new(xenon_position.x, xenon_position.y, xenon_radius);
        Xenon { xenon_elements }
    }
    pub fn try_absorption(&self, neutron: &Neutron) -> bool {
        neutron.is_thermal() && self.xenon_elements.contains(&neutron.position)
    }
    pub fn draw(&self) {
        draw_circle(self.xenon_elements.x, self.xenon_elements.y, self.xenon_elements.r, DARKGRAY);
    }
}