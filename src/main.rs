use bevy::prelude::*;
use bevy_easings::*;

#[derive(Component, Clone, Copy, Debug)]
struct Player;

#[derive(Component, Clone, Copy, Debug)]
struct Enemy;

#[derive(Component, Clone, Copy, Debug)]
struct Velocity(Vec2);

#[derive(Component, Clone, Copy, Debug)]
struct Stat {
    speed: f32,
}

#[derive(Component, Clone, Copy, Debug)]
struct KillOnBounds;

#[derive(Component, Clone, Copy, Debug)]
struct WrapOnBounds;

#[derive(Component, Clone, Copy, Debug)]
struct BlockOnBounds;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Shooter".to_string(),
            width: 1024.,
            height: 768.,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_easings::EasingsPlugin)
        .add_startup_system(setup)
        .add_system(keyboard_input)
        .add_system(movement)
        .add_system(kill_on_bounds)
        .add_system(wrap_on_bounds)
        .add_system(block_on_bounds)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    println!("{:?}", window.width());
    let player_position = Vec3::new(0.0, -window.height() / 2.0 + 20.0, 1.);
    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    // player
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..Default::default()
            },
            transform: Transform {
                translation: player_position,
                ..Default::default()
            },
            texture: asset_server.load("Angel_126x144.png"),
            ..Default::default()
        })
        .insert(Player)
        .insert(Velocity(Vec2::new(0., 0.)))
        .insert(WrapOnBounds)
        .insert(BlockOnBounds)
        .insert(Stat { speed: 3. });

    // enemy
    let enemy_position = Vec3::new(0.0, window.height() / 2.0 - 20.0, 1.);
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(20.0, 20.0)),
                flip_y: true,
                ..Default::default()
            },
            transform: Transform {
                translation: enemy_position,
                scale: Vec3::new(-0.5, -0.5, 1.0),
                ..Default::default()
            },
            texture: asset_server.load("OverlordOP2020D.png"),
            ..Default::default()
        })
        .insert(Enemy)
        .insert(KillOnBounds)
        .insert(Transform::from_translation(enemy_position).ease_to(
            Transform::from_xyz(0.0, (-window.height() * 0.5) - (20.0 * 0.5), 0.0),
            EaseMethod::CustomFunction(|x| x),
            bevy_easings::EasingType::Once {
                duration: std::time::Duration::from_secs(1),
            },
        ));
}

fn keyboard_input(
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

fn movement(mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation.x += velocity.0.x;
        transform.translation.y += velocity.0.y;
    }
}

fn kill_on_bounds(
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

fn wrap_on_bounds(
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

fn block_on_bounds(
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
