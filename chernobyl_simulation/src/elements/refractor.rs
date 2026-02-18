use crate::elements::neutron::*;
use macroquad::prelude::*;
use ::rand::Rng;

const REFRACTOR_MARGIN: f32 = 20.0;
pub struct Refractor {
    pub margin: f32,
    pub left: Rect,
    pub right: Rect,
    pub upper: Rect,
    pub lower: Rect,
}
impl Refractor {
    pub fn new() -> Refractor {
        let rect: Rect = Rect::new(0.0, 0.0, 0.0, 0.0);
        let refractor_vec: Vec<Rect> = vec![rect, rect, rect, rect];
        Refractor { margin: REFRACTOR_MARGIN, left: refractor_vec[0], right: refractor_vec[1], upper: refractor_vec[2], lower: refractor_vec[3] }
    }
    pub fn update_size(&mut self) {
        let width: f32 = screen_width() - 2.0 * self.margin;

        self.left.x = 0.0;
        self.left.y = 0.0;
        self.left.w = self.margin;
        self.left.h = screen_height();

        self.right.x = screen_width() - self.margin;
        self.right.y = 0.0;
        self.right.w = self.margin;
        self.right.h = screen_height();

        self.upper.x = self.margin;
        self.upper.y = 0.0;
        self.upper.w = width;
        self.upper.h = self.margin;

        self.lower.x = self.margin;
        self.lower.y = screen_height() - self.margin;
        self.lower.w = width;
        self.lower.h = self.margin;
    }
    pub fn draw(&self) {
        draw_rectangle(self.left.x, self.left.y, self.left.w, self.left.h, DARKGRAY);
        draw_rectangle(self.right.x, self.right.y, self.right.w, self.right.h, DARKGRAY);
        draw_rectangle(self.upper.x, self.upper.y, self.upper.w, self.upper.h, DARKGRAY);
        draw_rectangle(self.lower.x, self.lower.y, self.lower.w, self.lower.h, DARKGRAY);
    }
    pub fn neutron_bounce(&self, neutron: &mut Neutron) -> bool {
        let mut bounce: bool = false;
        if self.left.contains(neutron.position) || self.right.contains(neutron.position) {
            neutron.direction.x = -neutron.direction.x;
            bounce = true;
        }
        else if self.upper.contains(neutron.position) || self.lower.contains(neutron.position) {
            neutron.direction.y = -neutron.direction.y;
            bounce = true;
        }
        if bounce {
            let mut rng: ::rand::prelude::ThreadRng = ::rand::thread_rng();
            let is_absorbed: bool = rng.gen_bool(0.25);
            neutron.run();
            return is_absorbed;
        }
        else {
            false
        }
    }
}