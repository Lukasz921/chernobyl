use macroquad::audio::{play_sound_once, Sound};
use crate::elements::u235type::*;
use crate::elements::reaction::*;
use crate::elements::neutron::*;
use macroquad::prelude::*;

const COOLDOWN: i32 = 1;
const DROP_COOLDOWN: i32 = 10;
const FRAME_RATE: i32 = 60;
const U235_RADIUS: f32 = 10.0;

#[derive(Clone)]
pub struct U235 {
    pub circle: Circle,
    pub u235type: U235Type,
    pub cooldown: i32,
}
impl U235 {
    pub fn new(x: f32, y: f32) -> U235 {
        let circle: Circle = Circle::new(x, y, U235_RADIUS);
        let u235type: U235Type = U235Type::Active;
        let cooldown: i32 = 0;
        U235 { circle, u235type ,cooldown }
    }
    pub fn draw(&self) {
        let color: Color = match self.u235type {
            U235Type::Active => BLUE,
            U235Type::PreXenon => DARKGREEN,
            U235Type::Xenon => GREEN,
            U235Type::Inactive => DARKBLUE,
        };
        draw_circle(self.circle.x, self.circle.y, U235_RADIUS, color);
    }
    pub fn reaction(&mut self, neutron: &mut Neutron, sound: &Sound) -> Reaction {
        if self.circle.contains(&neutron.position) {
            if self.u235type == U235Type::Active && neutron.is_thermal {
                self.u235type = U235Type::PreXenon;
                self.cooldown = FRAME_RATE * COOLDOWN;
                play_sound_once(sound);
                return Reaction::Fission(Vec2::new(self.circle.x, self.circle.y));
            }
            else if self.u235type == U235Type::Xenon {
                self.u235type = U235Type::Inactive;
                self.cooldown = FRAME_RATE * COOLDOWN;
                return Reaction::Poison;
            }
        }
        Reaction::None
    }
    pub fn update_cooldown(&mut self) {
        match self.u235type {
            U235Type::PreXenon => {
                if self.cooldown == 0 {
                    self.u235type = U235Type::Xenon;
                    self.cooldown = FRAME_RATE * DROP_COOLDOWN;
                }
                else {
                    self.cooldown -= 1;
                }
            }
            U235Type::Inactive => {
                if self.cooldown == 0 {
                    self.u235type = U235Type::Active;
                }
                else {
                    self.cooldown -= 1;
                }
            }
            U235Type::Xenon => {
                if self.cooldown == 0 {
                    self.u235type = U235Type::Inactive;
                    self.cooldown = FRAME_RATE * COOLDOWN;
                }
                else {
                    self.cooldown -= 1;
                }
            }
            _ => {}
        }
    }
}