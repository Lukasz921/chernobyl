use crate::elements::moderator_rod::*;
use crate::elements::control_rod::*;
use crate::elements::uranium::*;
use crate::elements::waste::*;
use crate::elements::water::*;
use crate::elements::fuel::*;
use macroquad::prelude::*;
use ::rand::prelude::*;
pub struct Core {
    pub fuel: Vec<Vec<Fuel>>,
    pub water: Vec<Vec<Water>>,
    pub elements_per_channel_width: usize,
    pub moderator_rods: Vec<ModeratorRod>,
    pub control_rods: Vec<ControlRod>,
    pub control_rods_percent: f32,
}
impl Core {
    pub fn new(fuel_channels_count: usize, elements_per_channel_width: usize, fuel_channel_height: usize, element_radius: f32,
        fuel_probability: f64, water_size: f32, min_temperature: usize, max_temperature: usize, boiling_temperature: usize, rod_thick: f32) -> Core {
        let fuel: Vec<Vec<Fuel>> = new_channels_with_fuel(fuel_channels_count, elements_per_channel_width,
        fuel_channel_height, element_radius, fuel_probability);
        let water: Vec<Vec<Water>> = new_water_elements(fuel_channels_count, elements_per_channel_width, fuel_channel_height,
        water_size, min_temperature, max_temperature, boiling_temperature);
        let elements_per_channel_width: usize = elements_per_channel_width;
        let moderator_rods: Vec<ModeratorRod> = new_moderation_rods(fuel_channels_count, rod_thick);
        let control_rods: Vec<ControlRod> = new_control_rods(fuel_channels_count, rod_thick);
        let control_rods_percent: f32 = 1.0;
        Core { fuel, water, elements_per_channel_width, moderator_rods, control_rods, control_rods_percent }
    }
    pub fn window_size_changed(&mut self) {
        let columns_count: usize = self.water[0].len(); 
        let rows_count: usize = self.water.len(); 
        let x_step: f32 = screen_width() / (columns_count + 1) as f32;
        let y_step: f32 = screen_height() / (rows_count + 1) as f32;
        for (c, fuel_col) in self.fuel.iter_mut().enumerate() {
            let x = (c as f32 + 1.0) * x_step; 
            for (r, fuel) in fuel_col.iter_mut().enumerate() {
                let y = (r as f32 + 1.0) * y_step; 
                match fuel {
                    Fuel::Uranium(element) => { element.uranium_element.x = x; element.uranium_element.y = y; }
                    Fuel::Xenon(element) => { element.xenon_elements.x = x; element.xenon_elements.y = y; }
                    Fuel::Waste(element) => { element.waste_elements.x = x; element.waste_elements.y = y; }
                }
            }
        }
        for (r, water_row) in self.water.iter_mut().enumerate() {
            let y = (r as f32 + 1.0) * y_step;
            
            for (c, water) in water_row.iter_mut().enumerate() {
                let x = (c as f32 + 1.0) * x_step;
                water.water_element.x = x;
                water.water_element.y = y;
            }
        }
        let rod_y: f32 = y_step * 0.5;
        let rod_h: f32 = (rows_count as f32) * y_step;
        let elements: f32 = self.elements_per_channel_width as f32;
        for (i, rod) in self.moderator_rods.iter_mut().enumerate() {
            let x_center: f32 = (i as f32 * elements + (elements / 2.0) + 0.5) * x_step;    
            rod.moderator_rod.x = x_center - (rod.moderator_rod.w / 2.0);
            rod.moderator_rod.y = rod_y;
            rod.moderator_rod.h = rod_h;
        }
        for (i, rod) in self.control_rods.iter_mut().enumerate() {
            let x_center: f32 = (i as f32 * elements + 0.5) * x_step; 
            rod.control_rod.x = x_center - (rod.control_rod.w / 2.0);
            let current_h: f32 = rod_h * self.control_rods_percent;
            rod.control_rod.h = current_h;
            if i % 2 == 0 {
                rod.control_rod.y = rod_y;
            } 
            else {
                rod.control_rod.y = (rod_y + rod_h) - current_h;
            }
        }
    }
    pub fn draw(&self, cold_color: Color, hot_color: Color) {
        for water_vec in &self.water {
            for water in water_vec {
                water.draw(cold_color, hot_color);
            }
        }
        for fuel_vec in &self.fuel {
            for fuel in fuel_vec {
                match fuel {
                    Fuel::Uranium(element) => { element.draw(); }
                    Fuel::Xenon(element) => { element.draw(); }
                    Fuel::Waste(element) => { element.draw(); }
                }
            }
        }
        for moderator_rod in &self.moderator_rods {
            moderator_rod.draw();
        }
        for control_rod in &self.control_rods {
            control_rod.draw();
        }
    }
}
fn new_channels_with_fuel(fuel_channels_count: usize, elements_per_channel_width: usize, fuel_channel_height: usize, element_radius: f32, 
    probability: f64) -> Vec<Vec<Fuel>> {
    let mut fuel: Vec<Vec<Fuel>> = Vec::new();
    for _ in 0..fuel_channels_count * elements_per_channel_width {
        let mut row: Vec<Fuel> = Vec::new();
        for _ in 0..fuel_channel_height {
            let waste_element: Waste = Waste::new(Vec2::new(0.0, 0.0), element_radius);
            let mut fuel_element: Fuel = Fuel::Waste(waste_element);
            let mut rng: ThreadRng = ::rand::thread_rng();
            let is_uranium: bool = rng.gen_bool(probability);
            if is_uranium { 
                let uranium_element: Uranium = Uranium::new(Vec2::new(0.0, 0.0), element_radius);
                fuel_element = Fuel::Uranium(uranium_element);
            }
            row.push(fuel_element);
        }
        fuel.push(row);
    }
    fuel
}
fn new_water_elements(fuel_channels_count: usize, elements_per_channel_width: usize, fuel_channel_height: usize, 
    water_size: f32, min_temperature: usize, max_temperature: usize, boiling_temperature: usize) -> Vec<Vec<Water>> {
    vec![vec![Water::new_with_move(Vec2::new(0.0, 0.0), water_size, min_temperature, max_temperature, boiling_temperature); 
    fuel_channels_count * elements_per_channel_width]; fuel_channel_height]
}
fn new_moderation_rods(fuel_channels_count: usize, rod_thick: f32) -> Vec<ModeratorRod> {
    vec![ModeratorRod::new(0.0, 0.0, rod_thick); fuel_channels_count]
}
fn new_control_rods(fuel_channels_count: usize, rod_thick: f32) -> Vec<ControlRod> {
    vec![ControlRod::new(0.0, 0.0, rod_thick); fuel_channels_count + 1]
}