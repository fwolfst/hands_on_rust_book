mod collisions;
mod end_turn;
mod entity_render;
mod hud;
mod map_renderer;
mod movement;
mod moving_randomly;
mod player_input;
mod tooltips;

use crate::prelude::*;

pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .flush()
        .add_system(map_renderer::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(tooltips::tooltip_system())
        .add_system(hud::hud_system())
        .build()
}

pub fn build_player_turn_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(movement::movement_system())
        .flush()
        .add_system(collisions::collisions_system())
        .add_system(map_renderer::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(tooltips::tooltip_system())
        .add_system(end_turn::end_turn_system())
        .build()
}


pub fn build_monster_turn_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(moving_randomly::moving_randomly_system())
        .flush()
        .add_system(movement::movement_system())
        .flush()
        //? :
        .add_system(collisions::collisions_system())
        .flush()
        .add_system(map_renderer::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(hud::hud_system())
        .add_system(tooltips::tooltip_system())
        .add_system(end_turn::end_turn_system())
        .build()
}
