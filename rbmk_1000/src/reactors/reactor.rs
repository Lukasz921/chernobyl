use crate::elements::neutron::*;
use crate::reactors::core::*;
use crate::elements::fuel::*;
use crate::elements::uranium::*;
use crate::elements::xenon::*;
use crate::elements::waste::*;
use macroquad::audio::Sound;
use macroquad::prelude::*;
pub struct Reactor {
    pub core: Core,
    pub neutrons: Vec<Neutron>,
}
impl Reactor {
    pub fn new(fuel_channels_count: usize, elements_per_channel_width: usize, 
    fuel_channel_height: usize, element_radius: f32, fuel_probability: f64, water_size: f32, 
    min_temperature: usize, max_temperature: usize, boiling_temperature: usize, rod_thick: f32) -> Reactor {
        let core: Core = Core::new(fuel_channels_count, elements_per_channel_width, fuel_channel_height, element_radius, fuel_probability,
        water_size, min_temperature, max_temperature, boiling_temperature, rod_thick);
        let neutrons: Vec<Neutron> = Vec::new(); 
        Reactor { core, neutrons }
    }
    pub fn run_neutrons_simulation(&mut self, total_velocity: f32, velocity_left_percent: f32, neutron_multiplyer: usize, probability_water: f64, probability_xenon: f64, 
    is_refractor_enabled: bool, sound: &Sound) {
        let mut neutrons_idx_to_delete: Vec<usize> = Vec::new();
        let mut neutrons_to_add: Vec<Neutron> = Vec::new();
        let mut possible_xenon_releases: Vec<(usize, usize)> = Vec::new();
        let mut xenon_strikes: Vec<(usize, usize)> = Vec::new();
        'mainlabel: for neutron_tuple in &mut self.neutrons.iter_mut().enumerate() {
            neutron_tuple.1.run();
            if is_refractor_enabled {
                neutron_tuple.1.reflect();
            }
            else if neutron_tuple.1.check_if_gone() {
                neutrons_idx_to_delete.push(neutron_tuple.0);
                continue;
            }
            for moderator_rod in &self.core.moderator_rods {
                if moderator_rod.try_neutron_collision(neutron_tuple.1, velocity_left_percent) {
                    continue 'mainlabel;
                }
            }
            for control_rod in &self.core.control_rods {
                if control_rod.try_absorption(neutron_tuple.1) {
                    neutrons_idx_to_delete.push(neutron_tuple.0);
                    continue 'mainlabel;
                }
            }
            for element_row in self.core.fuel.iter_mut().enumerate() {
                for element in element_row.1.iter().enumerate() {
                    match element.1 {
                        Fuel::Uranium(uranium) => {
                            let new_possible_neutrons: Option<Vec<Neutron>> = uranium.try_fission(neutron_tuple.1, total_velocity, neutron_multiplyer, sound);
                            if let Some(mut new_neutrons) = new_possible_neutrons {
                                neutrons_to_add.append(&mut new_neutrons);
                                neutrons_idx_to_delete.push(neutron_tuple.0);
                                possible_xenon_releases.push((element_row.0, element.0));
                                continue 'mainlabel;
                            }
                        }
                        Fuel::Xenon(xenon) => {
                            if xenon.try_absorption(neutron_tuple.1) {
                                neutrons_idx_to_delete.push(neutron_tuple.0);
                                xenon_strikes.push((element_row.0, element.0));
                                continue 'mainlabel;
                            }
                        }
                        _ => {}
                    }
                }
            }
            for element_row in &mut self.core.water {
                for element in element_row {
                    element.try_get_hotter(neutron_tuple.1);
                    if element.try_absorption(neutron_tuple.1, probability_water) {
                        neutrons_idx_to_delete.push(neutron_tuple.0);
                        continue 'mainlabel;
                    }
                }
            }
        }
        neutrons_idx_to_delete.sort();
        neutrons_idx_to_delete.dedup();
        neutrons_idx_to_delete.reverse();
        for idx in neutrons_idx_to_delete {
            self.neutrons.remove(idx);
        }
        self.neutrons.append(&mut neutrons_to_add);
        self.xenon_release(possible_xenon_releases, probability_xenon);
        self.xenon_burn(xenon_strikes);
    }
    fn xenon_release(&mut self, possible_xenon_releases: Vec<(usize, usize)>, probability: f64) {
        for idx in possible_xenon_releases {
            let fuel_cell = &mut self.core.fuel[idx.0][idx.1];
            match fuel_cell {
                Fuel::Uranium(uranium) => {
                    if uranium.try_turn_to_xenon(probability) {
                        let xenon: Xenon = Xenon::new(Vec2::new(uranium.uranium_element.x, uranium.uranium_element.y), uranium.uranium_element.r);
                        *fuel_cell = Fuel::Xenon(xenon);
                    }
                    else {
                        let waste: Waste = Waste::new(Vec2::new(uranium.uranium_element.x, uranium.uranium_element.y), uranium.uranium_element.r);
                        *fuel_cell = Fuel::Waste(waste);
                    }
                }
                _ => {}
            }
        }
    }
    fn xenon_burn(&mut self, xenon_strikes: Vec<(usize, usize)>) {
        for idx in xenon_strikes {
            let fuel_cell = &mut self.core.fuel[idx.0][idx.1];
            match fuel_cell {
                Fuel::Xenon(xenon) => {
                    let waste: Waste = Waste::new(Vec2::new(xenon.xenon_elements.x, xenon.xenon_elements.y), xenon.xenon_elements.r);
                    *fuel_cell = Fuel::Waste(waste);
                }
                _ => {}
            }
        }
    }
    pub fn time_events(&mut self, spontaneous_neutron_throw_probability: f64, turning_to_uranium_probability: f64, total_velocity: f32, frame_number: usize, 
    spontaneous_neutron_throw_frames: &Vec<usize>, cool_down_frames: &Vec<usize>, try_turn_to_uranium_frames: &Vec<usize>, is_spontaneous_neutron_throw_enabled: bool) {
        if is_spontaneous_neutron_throw_enabled && spontaneous_neutron_throw_frames.contains(&frame_number) {
            let mut neutrons_to_add: Vec<Neutron> = Vec::new();
            for element_row in &self.core.fuel {
                for element in element_row {
                    match element {
                        Fuel::Waste(waste) => {
                            if let Some(neutron) = waste.spontaneous_neutron_throw(spontaneous_neutron_throw_probability, total_velocity) {
                                neutrons_to_add.push(neutron);
                            }
                        }
                        _ => {}
                    }
                }
            }
            self.neutrons.append(&mut neutrons_to_add);
        }
        if cool_down_frames.contains(&frame_number) {
            for element_row in &mut self.core.water {
                for element in element_row {
                    element.cool_down();
                }
            }
        }
        if try_turn_to_uranium_frames.contains(&frame_number) {
            for element_row in &mut self.core.fuel {
                for element in element_row {
                    match element {
                        Fuel::Waste(waste) => {
                            if waste.try_turn_to_uranium(turning_to_uranium_probability) {
                                let uranium: Uranium = Uranium::new(Vec2::new(waste.waste_elements.x, waste.waste_elements.y), waste.waste_elements.r);
                                *element = Fuel::Uranium(uranium);
                            }
                        }
                        _ => {}
                    }
                }
            }
        } 
    }
    pub fn window_size_changed(&mut self) {
        self.core.window_size_changed();
    }
    pub fn control_rods_move(&mut self) {
        if is_key_down(KeyCode::Up) {
            self.core.control_rods_percent -= 0.001;
            if self.core.control_rods_percent < 0.0 { self.core.control_rods_percent = 0.0 }
        }
        else if is_key_down(KeyCode::Down) {
            self.core.control_rods_percent += 0.001;
            if 1.0 < self.core.control_rods_percent { self.core.control_rods_percent = 1.0 }
        }
    }
    pub fn automated_control_rods(&mut self, lower_limit: usize, upper_limit: usize) {
        if self.neutrons.len() < lower_limit {
            self.core.control_rods_percent -= 0.001;
            if self.core.control_rods_percent < 0.0 { self.core.control_rods_percent = 0.0 }
        }
        else if upper_limit < self.neutrons.len() {
            self.core.control_rods_percent += 0.001;
            if 1.0 < self.core.control_rods_percent { self.core.control_rods_percent = 1.0 }
        }
        if lower_limit < self.neutrons.len() && self.neutrons.len() < upper_limit {
            if self.neutrons.len() - lower_limit < upper_limit - self.neutrons.len() {
                self.core.control_rods_percent -= 0.0001;
                if self.core.control_rods_percent < 0.0 { self.core.control_rods_percent = 0.0 }
            }
            else {
                self.core.control_rods_percent += 0.0001;
                if 1.0 < self.core.control_rods_percent { self.core.control_rods_percent = 1.0 }
            }
        }
    }
    pub fn az_5(&mut self) {
        self.core.control_rods_percent += 0.005;
        if 1.0 < self.core.control_rods_percent { self.core.control_rods_percent = 1.0; }
    }
    pub fn starting_neutrons_throw(&mut self, neutron_multiplyer: usize, neutron_radius: f32, total_velocity: f32, fuel_channels_count: usize) {
        if is_key_pressed(KeyCode::Space) {
            let x_step: f32 = self.core.control_rods[1].control_rod.x - self.core.control_rods[0].control_rod.x;
            let mut x1: f32 = screen_width() / 2.0;
            let mut x2: f32 = x1;
            let y: f32 = screen_height() / 2.0;
            let mut starting_neutrons: Vec<Neutron> = Vec::new();
            for _ in 0..fuel_channels_count {
                for i in 0..neutron_multiplyer {
                    let new_neutron: Neutron = Neutron::new_with_move(Vec2::new(x1, y), neutron_radius, total_velocity);
                    if i != 0 { 
                        let new_neutron: Neutron = Neutron::new_with_move(Vec2::new(x2, y), neutron_radius, total_velocity);
                        starting_neutrons.push(new_neutron);
                    }
                    starting_neutrons.push(new_neutron);
                }
                x1 += x_step;
                x2 -= x_step;
            }
            self.neutrons.append(&mut starting_neutrons);
        }
    }
    pub fn draw(&mut self, cold_color: Color, hot_color: Color, neutron_radius: f32) {
        self.core.draw(cold_color, hot_color);
        for neutron in &self.neutrons {
            neutron.draw(neutron_radius);
        }
    }
}