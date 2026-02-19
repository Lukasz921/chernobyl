use macroquad::prelude::*;
use ::rand::prelude::*;
#[derive(Clone)]
pub struct Neutron {
    pub position: Vec2,
    pub direction: Vec2,
    pub thermal: bool,
}
impl Neutron {
    pub fn new_with_move(element_position: Vec2, element_radius: f32, total_velocity: f32) -> Neutron {
        let direction: Vec2 = new_random_direction(total_velocity);
        let time: f32 = element_radius / total_velocity;
        let mut position: Vec2 = Vec2::new(element_position.x + (direction.x * time) + 1.0, 
        element_position.y + (direction.y * time) + 1.0);
        if direction.x < 0.0 { position.x -= 2.0 }
        if direction.y < 0.0 { position.y -= 2.0 }
        let thermal: bool = false;
        Neutron { position, direction, thermal }
    }
    pub fn make_thermal(&mut self, velocity_left_percent: f32) {
        if self.is_not_thermal() {
            self.direction.x = self.direction.x * velocity_left_percent;
            self.direction.y = self.direction.y * velocity_left_percent;
        }
    }
    pub fn is_thermal(&self) -> bool {
        self.thermal
    }
    pub fn is_not_thermal(&self) -> bool {
        !self.thermal
    }
    pub fn run(&mut self) {
        self.position.x += self.direction.x;
        self.position.y += self.direction.y;
    }
    pub fn check_if_gone(&self) -> bool {
        self.position.x < 0.0 || self.position.y < 0.0 || screen_width() < self.position.x
        || screen_height() < self.position.y
    }
    pub fn draw(&self, neutron_radius: f32) {
        if self.is_thermal() {
            draw_circle(self.position.x, self.position.y, neutron_radius, BLACK);
        }
        else {
            draw_circle(self.position.x, self.position.y, neutron_radius, WHITE);
            draw_circle_lines(self.position.x, self.position.y, neutron_radius, 1.0, BLACK);
        }
    }
}
fn new_random_direction(total_velocity: f32) -> Vec2 {
    let mut rng: ThreadRng = ::rand::thread_rng();
    let dx: f32 = rng.gen_range(-total_velocity..=total_velocity);
    let mut dy: f32 = ((total_velocity * total_velocity) - (dx * dx)).sqrt();
    let dy_up: bool = rng.gen_bool(0.5);
    if dy_up { dy = -dy }
    return Vec2::new(dx, dy)
}