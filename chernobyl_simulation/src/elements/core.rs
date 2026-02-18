use crate::elements::reaction::*;
use crate::elements::neutron::*;
use crate::elements::u235::*;
use macroquad::prelude::*;
use macroquad::audio::Sound;

const U235_RADIUS: f32 = 10.0;
const ROD_COUNT: usize = 5;
const ROD_HEIGHT: usize = 15;
const NEUTRON_MULTIPLYER: usize = 3;
const CONTROL_ROD_THICK: f32 = 20.0;
const GRAPHITE_ROD_THICK: f32 = 20.0;
const CONTROL_ROD_HEIGHT_MULTIPLYER: f32 = 0.33;

pub struct Core {
    pub fuel_rods: Vec<Vec<U235>>,
    pub graphite_rods: Vec<Rect>,
    pub control_rods: Vec<Rect>,
    pub control_rod_height_multiplyer: f32,
    pub chernobyl_disaster: bool,
}
impl Core {
    pub fn new() -> Core {
        let mut fuel_rods: Vec<Vec<U235>> = Vec::new();
        for _ in 0..ROD_HEIGHT {
            let rod: Vec<U235> = vec![U235::new(0.0, 0.0); (ROD_COUNT - 1) * 4];
            fuel_rods.push(rod);
        }
        let graphite_rods: Vec<Rect> = vec![Rect::new(0.0, 0.0, 0.0, 0.0); ROD_COUNT - 1];
        let control_rods: Vec<Rect> = vec![Rect::new(0.0, 0.0, 0.0, 0.0); ROD_COUNT];
        let control_rod_height_multiplyer: f32 = CONTROL_ROD_HEIGHT_MULTIPLYER;
        let chernobyl_disaster: bool = false;
        Core { fuel_rods, graphite_rods, control_rods, control_rod_height_multiplyer, chernobyl_disaster }
    }
    pub fn update_size(&mut self) {
        let control_rod_x_step: f32 = screen_width() / (ROD_COUNT as f32 + 1.0);
        let graphite_rod_x_step: f32 = 1.5 * control_rod_x_step;
        let y_step: f32 = screen_height() / (ROD_HEIGHT as f32 + 1.0);

        let control_height: f32 = screen_height() * self.control_rod_height_multiplyer;
        let graphite_height: f32 = (y_step * (ROD_HEIGHT as f32 - 1.0)) + (U235_RADIUS * 2.0);

        let mut x: f32 = control_rod_x_step - (CONTROL_ROD_THICK / 2.0);
        let y: f32 = y_step - U235_RADIUS;

        for i in 0..self.control_rods.len() {
            self.control_rods[i].x = x;
            if i.is_multiple_of(2) { 
                self.control_rods[i].y = y;
                self.control_rods[i].h = control_height;
            }
            else { 
                let bottom_y = y_step + graphite_height - U235_RADIUS;
                self.control_rods[i].y = bottom_y - control_height;
                self.control_rods[i].h = control_height;
            }
            self.control_rods[i].w = CONTROL_ROD_THICK;
            x += control_rod_x_step;
        }

        let mut x: f32 = graphite_rod_x_step - (GRAPHITE_ROD_THICK / 2.0);
        let y: f32 = y_step - U235_RADIUS;
        for rod in &mut self.graphite_rods {
            rod.x = x;
            rod.y = y;
            rod.w = GRAPHITE_ROD_THICK;
            rod.h = graphite_height;
            x += control_rod_x_step;
        }

        let mut y: f32 = y_step;
        let x_add: f32 = (graphite_rod_x_step - control_rod_x_step) / 3.0;
        for i in 0..ROD_HEIGHT {
            let mut x1: f32 = control_rod_x_step + x_add;
            let mut x2: f32 = graphite_rod_x_step + x_add;
            let mut idx: usize = 0;
            for _ in 0..self.graphite_rods.len() {
                self.fuel_rods[i][idx].circle.x = x1;
                self.fuel_rods[i][idx].circle.y = y;
                self.fuel_rods[i][idx + 1].circle.x = x1 + x_add;
                self.fuel_rods[i][idx + 1].circle.y = y;

                self.fuel_rods[i][idx + 2].circle.x = x2;
                self.fuel_rods[i][idx + 2].circle.y = y;
                self.fuel_rods[i][idx + 3].circle.x = x2 + x_add;
                self.fuel_rods[i][idx + 3].circle.y = y;
                idx += 4;
                x1 += control_rod_x_step;
                x2 += control_rod_x_step;
            }
            y += y_step;
        }
    }
    pub fn update_cooldown(&mut self) {
        for rod in &mut self.fuel_rods {
            for u235 in rod {
                u235.update_cooldown();
            }
        }
    }
    pub fn update_control_rod_size(&mut self, b: bool) {
        let y_step: f32 = screen_height() / (ROD_HEIGHT as f32 + 1.0);
        let graphite_height: f32 = (y_step * (ROD_HEIGHT as f32 - 1.0)) + (U235_RADIUS * 2.0);
        if b {
            self.control_rod_height_multiplyer += 0.01;
            if screen_height() * self.control_rod_height_multiplyer > graphite_height {
                self.control_rod_height_multiplyer = graphite_height / screen_height();
            }
        }
        else {
            self.control_rod_height_multiplyer -= 0.01;
            if self.control_rod_height_multiplyer < 0.0 {
                self.control_rod_height_multiplyer = 0.0;
            }
        }
    }
    pub fn automated_control_rod_size(&mut self, neutron_count: usize, frame_number: usize) {
        if frame_number == 30 {
            if neutron_count < 400 {
                self.control_rod_height_multiplyer -= 0.01;
                if self.control_rod_height_multiplyer < 0.0 {
                    self.control_rod_height_multiplyer = 0.0;
                }
            }
            else if neutron_count > 600 {
                let y_step: f32 = screen_height() / (ROD_HEIGHT as f32 + 1.0);
                let graphite_height: f32 = (y_step * (ROD_HEIGHT as f32 - 1.0)) + (U235_RADIUS * 2.0);
                self.control_rod_height_multiplyer += 0.01;
                if screen_height() * self.control_rod_height_multiplyer > graphite_height {
                    self.control_rod_height_multiplyer = graphite_height / screen_height();
                }
            }
        }
    }
    pub fn draw(&self) {
        for rod in &self.fuel_rods {
            for u235 in rod {
                u235.draw();
            }
        }
        for rod in &self.graphite_rods {
            draw_rectangle(rod.x, rod.y, rod.w, rod.h, GRAY);
        }
        let mut color: Color = BLACK;
        if self.chernobyl_disaster { color = GRAY }
        for rod in &self.control_rods {
            draw_rectangle(rod.x, rod.y, rod.w, rod.h, color);
        }
    }
    pub fn moderate(&mut self, neutron_vec: &mut Vec<Neutron>) {
        for neutron in neutron_vec {
            for rod in &mut self.graphite_rods {
                if rod.contains(neutron.position) {
                    neutron.make_thermal();
                    break;
                }
            }
        }
    }
    pub fn absorb(&mut self, neutron_vec: &mut Vec<Neutron>) {
        if self.chernobyl_disaster {
            self.moderate(neutron_vec);
        }
        else {
            let mut to_delete: Vec<usize> = Vec::new();
            for neutron_tuple in neutron_vec.iter_mut().enumerate() {
                for rod in &mut self.control_rods {
                    if rod.contains(neutron_tuple.1.position) {
                        to_delete.push(neutron_tuple.0);
                        break;
                    }
                }
            }
            to_delete.sort();
            to_delete.reverse();
            for idx in to_delete {
                neutron_vec.remove(idx);
            }
        }
    }
    pub fn reaction(&mut self, neutron_vec: &mut Vec<Neutron>, sound: &Sound) -> usize {
        let mut to_delete: Vec<usize> = Vec::new();
        let mut to_add: Vec<Vec2> = Vec::new();
        for neutron_tuple in neutron_vec.iter_mut().enumerate() {
            'label: for rod in &mut self.fuel_rods {
                for u235 in rod {
                    match u235.reaction(neutron_tuple.1, sound) {
                        Reaction::Fission(position) => {
                            to_delete.push(neutron_tuple.0);
                            to_add.push(position);
                            break 'label;
                        }
                        Reaction::Poison => {
                            to_delete.push(neutron_tuple.0);
                            break 'label;
                        }
                        Reaction::None => {}
                    }
                }
            }
        }
        to_delete.sort();
        to_delete.reverse();
        for idx in to_delete {
            neutron_vec.remove(idx);
        }
        let len: usize = to_add.len();
        for position in to_add {
            for _ in 0..NEUTRON_MULTIPLYER {
                neutron_vec.push(Neutron::new_from_reaction(position.x, position.y));
            }
        }
        return len;
    }
}