use crate::elements::u235type::*;
use crate::elements::neutron::*;
use crate::elements::core::*;
use macroquad::prelude::*;

const FRAME_RATE: usize = 60;

pub struct Counter {
    pub total_neutrons: usize,
    pub total_active_u235: usize,
    pub total_poisoned_u235: usize,
    pub fision_accumulator: usize,
    pub display_fission_rate: usize,
    pub frame_number: usize,
}

impl Counter {
    pub fn new() -> Counter {
        Counter {
            total_neutrons: 0,
            total_active_u235: 0,
            total_poisoned_u235: 0,
            fision_accumulator: 0,
            display_fission_rate: 0,
            frame_number: 0,
        }
    }
    pub fn work(&mut self, neutron_vec: &mut Vec<Neutron>, core: &Core, fission_per_frame: usize) -> usize {
        self.total_neutrons = neutron_vec.len();
        self.fision_accumulator += fission_per_frame;
        self.frame_number += 1;   
        if self.frame_number >= FRAME_RATE {
            self.display_fission_rate = self.fision_accumulator;
            self.fision_accumulator = 0;
            self.frame_number = 0;
        }
        self.total_active_u235 = 0;
        self.total_poisoned_u235 = 0;
        for rod in &core.fuel_rods {
            for u235 in rod {
                match u235.u235type {
                    U235Type::Active => self.total_active_u235 += 1,
                    U235Type::Xenon => self.total_poisoned_u235 += 1,
                    _ => {}
                }
            }
        }
        return self.frame_number;
    }

    pub fn draw(&self) {
        let panel_x = 20.0;
        let panel_y = 20.0;
        let width = 300.0;
        let height = 140.0;
        let font_size = 30.0;
        let line_height = 30.0;
        draw_rectangle(panel_x, panel_y, width, height, Color::new(0.0, 0.0, 0.0, 0.7));
        draw_rectangle_lines(panel_x, panel_y, width, height, 2.0, PURPLE);
        let text_color = Color::new(0.8, 0.5, 1.0, 1.0);
        draw_text(
            &format!("Active U235: {}", self.total_active_u235),
            panel_x + 10.0,
            panel_y + 30.0,
            font_size,
            text_color,
        );
        draw_text(
            &format!("Xenon Poison: {}", self.total_poisoned_u235),
            panel_x + 10.0,
            panel_y + 30.0 + line_height,
            font_size,
            text_color,
        );
        draw_text(
            &format!("Neutrons: {}", self.total_neutrons),
            panel_x + 10.0,
            panel_y + 30.0 + (line_height * 2.0),
            font_size,
            text_color,
        );
        draw_text(
            &format!("Fission/s: {}", self.display_fission_rate),
            panel_x + 10.0,
            panel_y + 30.0 + (line_height * 3.0),
            font_size,
            text_color,
        );
        draw_text(
            &format!("Power: {} MW", self.display_fission_rate * 32),
            panel_x + 10.0,
            panel_y + 30.0 + (line_height * 4.0),
            font_size,
            text_color,
        );
    }
}