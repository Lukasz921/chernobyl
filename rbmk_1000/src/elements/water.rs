use crate::elements::neutron::*;
use macroquad::prelude::*;
use ::rand::prelude::*;

#[derive(Clone)]
pub struct Water {
    pub water_element: Rect,
    pub min_temperature: usize,
    pub max_temperature: usize,
    pub boiling_temperature: usize,
    pub curr_temperature: usize,
}
impl Water {
    pub fn new_with_move(element_position: Vec2, water_size: f32, min_temperature: usize, max_temperature: usize, boiling_temperature: usize)
    -> Water {
        let x: f32 = element_position.x - (water_size / 2.0);
        let y: f32 = element_position.y - (water_size / 2.0);
        let water_element: Rect = Rect::new(x, y, water_size, water_size);
        let curr_temperature: usize = min_temperature;
        Water { water_element, min_temperature, max_temperature, boiling_temperature, curr_temperature }
    }
    pub fn try_get_hotter(&mut self, neutron: &Neutron) {
        if self.water_element.contains(neutron.position) && self.curr_temperature < self.max_temperature {
            self.curr_temperature += 1;
        }
    }
    pub fn cool_down(&mut self) {
        if self.min_temperature < self.curr_temperature {
            self.curr_temperature -= 1;
        }
    }
    pub fn try_absorption(&self, neutron: &Neutron, probability: f64) -> bool {
        if self.water_element.contains(neutron.position) && self.curr_temperature < self.boiling_temperature {
            let mut rng: ThreadRng = ::rand::thread_rng();
            return rng.gen_bool(probability);
        }
        false
    }
    pub fn draw(&self, cold_color: Color, hot_color: Color) {
        if self.curr_temperature <= self.boiling_temperature {
            let t: f32 = (self.curr_temperature - self.min_temperature) as f32 / 
            (self.boiling_temperature - self.min_temperature) as f32;
            let t: f32 = t.clamp(0.0, 1.0);
            let water_color: Color = color_lerp(cold_color, hot_color, t);
            let offset: f32 = self.water_element.w / 2.0;
            draw_rectangle(self.water_element.x - offset, self.water_element.y - offset,
            self.water_element.w, self.water_element.h, water_color);
        }
    }
}
fn color_lerp(a: Color, b: Color, t: f32) -> Color {
    Color::new(
        lerp(a.r, b.r, t),
        lerp(a.g, b.g, t),
        lerp(a.b, b.b, t),
        lerp(a.a, b.a, t),
    )
}
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}