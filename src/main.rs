use bevy::prelude::*;
use bevy::ui::AlignSelf::Start;

#[cfg(not(feature = "reload"))]
use systems::*;
#[cfg(feature = "reload")]
use systems_hot::*;

#[cfg(feature = "reload")]
#[hot_lib_reloader::hot_module(dylib = "systems")]
mod systems_hot {
    use bevy::prelude::*;
    pub use components::*;
    hot_functions_from_file!("systems/src/lib.rs");
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_systems(PostStartup, systems::setup)
        .add_systems(Update, (
            player_movement_system,
            player_shooting_system,
            bullet_movement_system,
            bullet_hit_system,
            spawn_other_ships,
            move_other_ships,
        ))
        .add_system(bevy::window::close_on_esc);

    app.run();
}
