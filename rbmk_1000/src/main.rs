use macroquad::audio::{load_sound, Sound};
use rbmk_1000::reactors::reactor::*;
use macroquad::prelude::*;

const FUEL_CHANNELS_COUNT: usize = 10; // 10
const ELEMENTS_PER_CHANNEL: usize = 4;
const FUEL_CHANNEL_HEIGHT: usize = 20; // 20
const ELEMENT_RADIUS: f32 = 10.0; // 7.5
const FUEL_PROBABILITY: f64 = 0.1;
const WATER_SIZE: f32 = 40.0; // 35.0
const MIN_TEMPERATURE: usize = 20;
const MAX_TEMPERATURE: usize = 200;
const BOILING_TEMPERATURE: usize = 100;
const ROD_THICK: f32 = 10.0;

const NEUTRON_RADIUS: f32 = 5.0;
const TOTAL_VELOCITY: f32 = 4.0;
const VELOCITY_LEFT_PERCENT: f32 = 1.0 / 3.0;
const NEUTRON_MULTIPLYER: usize = 3;
const PROBABILITY_WATER: f64 = 0.005;
const PROBABILITY_XENON: f64 = 0.1;

const SPONTANEOUS_NEUTRON_THROW_PROBABILITY: f64 = 0.01;
const TURNING_TO_URANIUM_PROBABILITY: f64 = 0.05;

const AUTOMATED_LOW_LIMIT: usize = 150;
const AUTOMATED_HIGH_LIMIT: usize = 250;

const POWER_PER_NEUTRON: f32 = 12.8; // 3200/250
const RMBK_MAX_POWER: f32 = 3200.0;
const CRITICAL_POWER: f32 = 4000.0;

#[macroquad::main("RBMK-1000")]
async fn main() {
    let sound: Sound = load_sound("assets/reaction.ogg").await.unwrap();
    let mut reactor: Reactor = Reactor::new(FUEL_CHANNELS_COUNT, ELEMENTS_PER_CHANNEL, FUEL_CHANNEL_HEIGHT, ELEMENT_RADIUS,
    FUEL_PROBABILITY, WATER_SIZE, MIN_TEMPERATURE, MAX_TEMPERATURE, BOILING_TEMPERATURE, ROD_THICK);
    let spontaneous_neutron_throw_frames: Vec<usize> = vec![60];
    let cool_down_frames: Vec<usize> = vec![5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60];
    let try_turn_to_uranium_frames: Vec<usize> = vec![60];
    let mut frame_number: usize = 0;
    let mut automated: bool = true;
    let mut is_spontaneous_neutron_throw_enabled: bool = true;
    let mut is_az5_enabled: bool = false;
    let mut is_refractor_enabled: bool = false;
    loop {
        clear_background(WHITE);
        reactor.window_size_changed();
        reactor.draw(SKYBLUE, RED, NEUTRON_RADIUS);
        reactor.run_neutrons_simulation(TOTAL_VELOCITY, VELOCITY_LEFT_PERCENT, NEUTRON_MULTIPLYER, PROBABILITY_WATER, PROBABILITY_XENON, is_refractor_enabled, &sound);
        reactor.time_events(SPONTANEOUS_NEUTRON_THROW_PROBABILITY, TURNING_TO_URANIUM_PROBABILITY, TOTAL_VELOCITY, frame_number, &spontaneous_neutron_throw_frames,
        &cool_down_frames, &try_turn_to_uranium_frames, is_spontaneous_neutron_throw_enabled);
        frame_number += 1;
        if 60 < frame_number {
            frame_number = 0;
        }
        reactor.control_rods_move();
        reactor.starting_neutrons_throw(NEUTRON_MULTIPLYER, NEUTRON_RADIUS, TOTAL_VELOCITY, FUEL_CHANNELS_COUNT);
        if is_key_pressed(KeyCode::A) {
            automated = !automated;
        }
        if is_key_pressed(KeyCode::S) {
            is_spontaneous_neutron_throw_enabled = !is_spontaneous_neutron_throw_enabled;
        }
        if is_key_pressed(KeyCode::Enter) {
            is_az5_enabled = !is_az5_enabled;
        }
        if is_key_pressed(KeyCode::R) {
            is_refractor_enabled = !is_refractor_enabled;
        }
        if automated { reactor.automated_control_rods(AUTOMATED_LOW_LIMIT, AUTOMATED_HIGH_LIMIT); }
        if is_az5_enabled { reactor.az_5(); }

        let power_mw: f32 = reactor.neutrons.len() as f32 * POWER_PER_NEUTRON;
        let neutrons_text: String = format!("NEUTRONS: {}", reactor.neutrons.len());
        let rods_text: String = format!("CONTROL RODS: {:.1}%", reactor.core.control_rods_percent * 100.0);
        let power_text: String = format!("THERMAL POWER: {:.0} MW", power_mw);
        let power_color = if power_mw > CRITICAL_POWER {
            RED
        } else if power_mw > RMBK_MAX_POWER {
            ORANGE
        } else {
            DARKGREEN
        };
        let font_size = 25.0;
        let y_pos = 20.0;
        draw_text(&neutrons_text, 20.0, y_pos, font_size, BLACK);
        let rods_text_width = measure_text(&rods_text, None, font_size as u16, 1.0).width;
        draw_text(&rods_text, (screen_width() - rods_text_width) / 2.0, y_pos, font_size, BLACK);
        let power_text_width = measure_text(&power_text, None, font_size as u16, 1.0).width;
        draw_text(&power_text, screen_width() - power_text_width - 20.0, y_pos, font_size, power_color);
        next_frame().await;
    }
}