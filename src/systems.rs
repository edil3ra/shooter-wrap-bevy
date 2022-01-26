use bevy::prelude::*;
use crate::components::*;


pub fn keyboard_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &Stat), With<Player>>,
) {
    let (mut player_velocity, stat) = query.get_single_mut().unwrap();
    player_velocity.0.x = 0.0;
    player_velocity.0.y = 0.0;
    let mut player_move = false;

    if keyboard_input.pressed(KeyCode::Left) {
        player_velocity.0.x = -stat.speed;
        player_move = true;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        player_velocity.0.x = stat.speed;
        player_move = true;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        player_velocity.0.y = -stat.speed;
        player_move = true;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        player_velocity.0.y = stat.speed;
        player_move = true;
    }

    if player_move {
        player_velocity.0 = player_velocity.0.normalize() * stat.speed;
    } else {
        player_velocity.0 = Vec2::new(0., 0.);
    }
}

pub fn movement(mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation.x += velocity.0.x;
        transform.translation.y += velocity.0.y;
    }
}

pub fn kill_on_bounds(
    mut commands: Commands,
    windows: Res<Windows>,
    query: Query<(Entity, &Transform, &Sprite), With<KillOnBounds>>,
) {
    let window = windows.get_primary().unwrap();
    for (entity, transform, sprite) in query.iter() {
        if transform.translation.y
            <= (-window.height() * 0.5) - (sprite.custom_size.unwrap().y * 0.5)
        {
            commands.entity(entity).despawn();
        }
    }
}

pub fn wrap_on_bounds(
    windows: Res<Windows>,
    mut query: Query<(&mut Transform, &Sprite), With<WrapOnBounds>>,
) {
    let window = windows.get_primary().unwrap();
    let window_left = -window.width() * 0.5;
    let window_right = window.width() * 0.5;
    for (mut transform, sprite) in query.iter_mut() {
        let sprite_width = sprite.custom_size.unwrap().x * 0.5;
        if transform.translation.x <= window_left + sprite_width {
            transform.translation.x = window_right - sprite_width
        } else if transform.translation.x >= window_right - sprite_width {
            transform.translation.x = window_left + sprite_width
        }
    }
}

pub fn block_on_bounds(
    windows: Res<Windows>,
    mut query: Query<(&mut Transform, &Sprite), With<BlockOnBounds>>,
) {
    let window = windows.get_primary().unwrap();
    let window_up = window.height() * 0.5;
    let window_down = -window.height() * 0.5;
    for (mut transform, sprite) in query.iter_mut() {
        let sprite_height = sprite.custom_size.unwrap().x * 0.5;
        if transform.translation.y >= window_up - sprite_height {
            transform.translation.y = window_up - sprite_height
        } else if transform.translation.y <= window_down + sprite_height {
            transform.translation.y = window_down + sprite_height
        }
    }
}
