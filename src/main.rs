use bevy::prelude::*;
use bevy::tasks::{IoTaskPool, TaskPool};
use bevy::ui::AlignSelf::Start;
use bevy_inspector_egui;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use rand::{Rng, thread_rng};

#[cfg(not(feature = "reload"))]
use systems::*;
#[cfg(feature = "reload")]
use systems_hot::*;

#[cfg(feature = "reload")]
#[hot_lib_reloader::hot_module(dylib = "systems")]
mod systems_hot {
    use bevy::prelude::*;
    pub use components::*;
    pub use utilities::*;
    hot_functions_from_file!("systems/src/lib.rs");
}


fn main() {
    //bevy::tasks::IoTaskPool::init(|| {
    //    bevy::tasks::TaskPool::new()
    //});

    let mut app = App::new();
    app
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(PostStartup, setup)
        .add_systems(Update, (
            player_input_system,
            player_movement_system.after(player_input_system),
            player_shooting_system,
            bullet_movement_system,
            bullet_hit_system,
            spawn_other_ships,
            move_other_ships,
        ))
        .add_system(bevy::window::close_on_esc);

    app.run();
}




use components::OtherShip;
use components::Player;
use components::Bullet;
use bevy::{log, prelude::*, sprite::collide_aabb};

use core::f32::consts::PI;

pub fn player_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Player)>,
) {

    let (mut ship) = query.single_mut();

    let mut rotation_factor = 0.0;
    let mut movement_factor = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        rotation_factor += 1.0;
        println!("left");
    }

    if keyboard_input.pressed(KeyCode::Right) {
        rotation_factor -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        movement_factor += 1.0;
    }

    ship.rotation_factor = rotation_factor;
    ship.movement_factor = movement_factor;
}

pub fn spawn_other_ships(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    others: Query<(Entity, &Transform), With<OtherShip>>,
    cam: Query<&Camera>,
) {
    let io_pool = IoTaskPool::init(|| {
        TaskPool::new()
    });

    const MARGIN: f32 = 30.0;
    const MIN_SHIP_COUNT: usize = 10;

    let screen_size =  Vec2{x:800.0, y:600.0};
    let mut other_ships_count = 0;

    for (entity, tfm) in others.iter() {
        if utilities::is_outside_bounds(
            tfm.translation.truncate(),
            (
                (-screen_size.x) - MARGIN,
                screen_size.y + MARGIN,
                screen_size.x + MARGIN,
                (-screen_size.y) - MARGIN,
            ),
        ) {
            commands.entity(entity).despawn();
        } else {
            other_ships_count += 1;
        }
    }

    if other_ships_count < MIN_SHIP_COUNT {
        let x = if thread_rng().gen::<bool>() {
            thread_rng().gen_range(((-screen_size.x) - MARGIN)..(-screen_size.x))
        } else {
            thread_rng().gen_range(screen_size.x..(screen_size.x + MARGIN))
        };
        let y = if thread_rng().gen::<bool>() {
            thread_rng().gen_range(((-screen_size.y) - MARGIN)..(-screen_size.y))
        } else {
            thread_rng().gen_range(screen_size.y..(screen_size.y + MARGIN))
        };
        let dir = thread_rng().gen_range(0.0f32..360.0f32);
        let mut transform = Transform::from_xyz(x, y, 0.0);
        transform.rotate_z(dir.to_radians());

        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("textures/simplespace/enemy_A.png"),
                transform,
                ..default()
            })
            .insert(OtherShip);
    }
}

#[no_mangle]
pub fn player_shooting_system(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    query: Query<&Transform, With<Player>>,
) {
    const SIZE: f32 = 10.0;

    if keyboard_input.just_pressed(KeyCode::Space) {
        if let Ok(tfm) = query.get_single() {
            for i in 0..3 {
                let shoot_dir = spread(tfm.rotation * Vec3::Y, 10.0);
                // print shoot dir:
                //log::info!("shoot_dir: {:?}", shoot_dir);

                let translation = Mat4::from_translation(tfm.translation);
                let mut rotation = tfm.rotation;
                let random_between_0_and_1 = rand::random::<f32>();
                let angle_offset = (random_between_0_and_1 - 0.5) * 2.0 * PI * 0.05 *1.0;
                rotation = rotation * Quat::from_rotation_z(angle_offset);
                let rotation_matrix = Mat4::from_quat(rotation);

                let scale = Mat4::from_scale(tfm.scale);

                //let bullet_transform = Transform::from_matrix(translation * rotation);

                let bullet_transform = Transform::from_matrix(translation * rotation_matrix * scale);
                commands
                    .spawn(SpriteBundle {
                        transform: bullet_transform,
                        sprite: Sprite {
                            color: Color::rgb(0.9, 0.8, 0.0),
                            custom_size: Some(Vec2::new(SIZE, SIZE)),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Bullet);
            }

            //commands
            //    .spawn(SpriteBundle {
            //        transform: *tfm,
            //        sprite: Sprite {
            //            color: Color::rgb(0.9, 0.8, 0.0),
            //            custom_size: Some(Vec2::new(SIZE, SIZE)),
            //            ..Default::default()
            //        },
            //        ..Default::default()
            //    })
            //    .insert(Bullet);
        }
    }
}

// takes in a closure
fn spread (aim_dir:Vec3, spread_degrees: f32) -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut spread = rng.gen_range(-spread_degrees..spread_degrees);
    let mut spread_dir = aim_dir;
    let rotation = Quat::from_rotation_z(spread.to_radians());
    spread_dir = rotation * spread_dir;
    return spread_dir;
}

#[no_mangle]
pub fn bullet_hit_system(
    mut commands: Commands,
    bullet_query: Query<&Transform, With<Bullet>>,
    ship_query: Query<(Entity, &Transform), With<OtherShip>>,
) {
    for bullet_tfm in bullet_query.iter() {
        for (entity, ship_tfm) in ship_query.iter() {
            if collide_aabb::collide(
                bullet_tfm.translation,
                Vec2::new(10.0, 10.0),
                ship_tfm.translation,
                Vec2::new(30.0, 30.0),
            )
                .is_some()
            {
                log::info!("BUUMMMM");
                commands.entity(entity).despawn();
                // create another 2 bullets in random dir
                let mut rng = rand::thread_rng();
                let mut spread = rng.gen_range(-180.0..180.0);
                let mut spread_dir = ship_tfm.rotation * Vec3::Y;
                let rotation = Quat::from_rotation_z(spread);
                spread_dir = rotation * spread_dir;
                let mut translation = Mat4::from_translation(ship_tfm.translation);
                let mut rotation = ship_tfm.rotation;
                let random_between_0_and_1 = rand::random::<f32>();
                let angle_offset = (random_between_0_and_1 - 0.5) * 2.0 * PI * 0.05;
                rotation = rotation * Quat::from_rotation_z(angle_offset);
                let rotation_matrix = Mat4::from_quat(rotation);
                let scale = Mat4::from_scale(ship_tfm.scale);
                translation = translation * rotation_matrix * scale;

                commands
                    .spawn(SpriteBundle {
                        transform: Transform::from_matrix(translation),
                        sprite: Sprite {
                            color: Color::rgb(0.9, 0.8, 0.0),
                            custom_size: Some(Vec2::new(10.0, 10.0)),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Bullet);
            }
        }
    }
}

#[no_mangle]
pub fn bullet_movement_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform), With<Bullet>>,
    cam: Query<&Camera>,
    time: Res<Time>,
) {
    let screen_size = Vec2{x:800.0, y:600.0};
    let speed = 800.0;
    for (entity, mut tfm) in &mut query {
        let x = tfm
            .rotation
            .mul_vec3(Vec3::new(0.0, speed * time.delta_seconds(), 0.0));
        tfm.translation += x;
        tfm.translation += x;

        if utilities::is_outside_bounds(
            tfm.translation.truncate(),
            (
                (-screen_size.x),
                screen_size.y,
                screen_size.x,
                (-screen_size.y),
            ),
        ) {
            log::info!("pufff");
            commands.entity(entity).despawn();
        }
    }
}


#[no_mangle]
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // player
    let ship_handle = asset_server.load("textures/simplespace/ship_C.png");
    commands
        .spawn(SpriteBundle {
            texture: ship_handle,
            ..default()
        })
        .insert(Player {
            velocity: Vec3::ZERO,
            rotation_speed: f32::to_radians(180.0),
            shooting_timer: None,
            movement_factor: 0.0,
            rotation_factor: 0.0,
        });
}


