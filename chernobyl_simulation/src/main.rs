use macroquad::audio::{load_sound, Sound};
use chernobyl_simulation::elements::refractor::*;
use chernobyl_simulation::elements::neutron::*;
use chernobyl_simulation::elements::counter::*;
use chernobyl_simulation::elements::core::*;
use macroquad::prelude::*;

#[macroquad::main("Chernobyl")]
async fn main() {
    let mut refractor: Refractor = Refractor::new();
    let mut neutron_vec: Vec<Neutron> = Vec::new();
    let mut counter: Counter = Counter::new();
    let mut core: Core = Core::new();
    let sound: Sound = load_sound("assets/reaction.ogg").await.unwrap();
    let mut automated_control_rods: bool = false;
    loop {
        clear_background(LIGHTGRAY);

        if is_key_pressed(KeyCode::Space) {
            for _ in 0..5 {
                neutron_vec.push(Neutron::new_initial());
            }
        }
        if is_key_down(KeyCode::Up) {
            core.update_control_rod_size(true);
        }
        if is_key_down(KeyCode::Down) {
            core.update_control_rod_size(false);
        }
        if is_key_pressed(KeyCode::B) {
            core.chernobyl_disaster = !core.chernobyl_disaster;
        }
        if is_key_pressed(KeyCode::A) {
            automated_control_rods = !automated_control_rods;
        }

        refractor.update_size();
        refractor.draw();

        core.update_size();
        core.update_cooldown();
        core.moderate(&mut neutron_vec);
        core.absorb(&mut neutron_vec);
        core.draw();

        let mut to_delete: Vec<usize> = Vec::new();
        for neutron_tuple in &mut neutron_vec.iter_mut().enumerate() {
            neutron_tuple.1.run();
            neutron_tuple.1.draw();
            if refractor.neutron_bounce(neutron_tuple.1) {
                to_delete.push(neutron_tuple.0)
            }
        }
        to_delete.sort();
        to_delete.reverse();
        for idx in to_delete {
            neutron_vec.remove(idx);
        }

        let fission_per_frame: usize = core.reaction(&mut neutron_vec, &sound);

        let frame_number: usize = counter.work(&mut neutron_vec, &core, fission_per_frame);
        counter.draw();

        if automated_control_rods {
            core.automated_control_rod_size(neutron_vec.len(), frame_number);
        }

        next_frame().await
    }
}