use bevy::prelude::*;
use crate::components::*;
use bevy_easings::*;


pub fn spawn_player(commands: &mut Commands, asset_server: &Res<AssetServer>, windows: &Res<Windows>) {
    let window = windows.get_primary().unwrap();
    let player_position = Vec3::new(0.0, -window.height() / 2.0 + 20.0, 1.);
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
}


pub fn spawn_enemy(commands: &mut Commands, asset_server: &Res<AssetServer>, windows: &Res<Windows>) {
    let window = windows.get_primary().unwrap();
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
