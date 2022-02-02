use bevy::prelude::*;

mod spawners;
mod components;
mod systems;
    
use spawners::*;

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
        .add_system(systems::keyboard_input)
        .add_system(systems::movement)
        .add_system(systems::kill_on_bounds)
        .add_system(systems::wrap_on_bounds)
        .add_system(systems::block_on_bounds)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, windows: Res<Windows>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    spawn_player(&mut commands, &asset_server, &windows);
    spawn_enemy(&mut commands, &asset_server, &windows);
}
