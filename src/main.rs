use bevy::prelude::*;

//#[cfg(not(feature = "reload"))]
//use systems::*;
//#[cfg(feature = "reload")]
//use systems_hot::*;

#[cfg(not(feature = "reload"))]
use hot_functions::*;
#[cfg(feature = "reload")]
use hot_functions_hot::*;

//use hot_functions;

#[cfg(feature = "reload")]
#[hot_lib_reloader::hot_module(dylib = "hot_functions")]
mod systems_hot {
    use bevy::prelude::*;
    //pub use components::*;
    //hot_functions_from_file!("systems/src/systems");
    hot_functions_from_file!("hot_functions/src/lib");
}

mod components;
use components::*;
mod systems;
use systems::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_startup_system(systems::setup)
        .add_system(player_movement_system)
        .add_system(player_shooting_system)
        .add_system(bullet_movement_system)
        .add_system(bullet_hit_system)
        .add_system(spawn_other_ships)
        .add_system(move_other_ships)

        .add_system(bevy::window::close_on_esc);

    app.run();
}
