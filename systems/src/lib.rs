use std::f32::consts::PI;
use bevy::{prelude::*, sprite::collide_aabb};
use components::*;
use rand::{thread_rng, Rng};

#[no_mangle]
pub fn player_movement_system_inner(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
    time: Res<Time>,
) {
    const SPEED: f32 = 300.0;


    //println!("sds");




    let (ship, mut transform) = query.single_mut();

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

    // update the ship rotation around the Z axis (perpendicular to the 2D plane of the screen)
    transform.rotate_z(rotation_factor * ship.rotation_speed * time.delta_seconds());

    // get the ship's forward vector by applying the current rotation to the ships initial facing vector
    let movement_direction = transform.rotation * Vec3::Y;
    // get the distance the ship will move based on direction, the ship's movement speed and delta time
    let movement_distance = movement_factor * SPEED * time.delta_seconds();
    // create the change in translation using the new movement direction and distance
    let translation_delta = movement_direction * movement_distance;
    // update the ship translation with our new translation delta
    transform.translation += translation_delta;

    // bound the ship within the invisible level bounds
    let extents = Vec3::from((BOUNDS / 2.0, 0.0));
    transform.translation = transform.translation.min(extents).max(-extents);
}


#[no_mangle]
pub fn physics_collision_detection_system(

)
{

}


#[no_mangle]
pub fn move_other_ships(time: Res<Time>, mut query: Query<&mut Transform, With<OtherShip>>) {
    const SPEED: f32 = 100.0;
    for mut tfm in &mut query {
        let x = tfm
            .rotation
            .mul_vec3(Vec3::new(0.0, SPEED * time.delta_seconds(), 0.0));

        tfm.translation += x;
    }
}
