use bevy::prelude::*;



#[derive(Component, Clone, Copy, Debug)]
struct Player;


#[derive(Component, Clone, Copy, Debug)]
struct Velocity(Vec2);

#[derive(Component, Clone, Copy, Debug)]
struct Stat{
    speed: f32
}


// impl Velocity {
//     fn new(x: f32, y: f32) -> Self {
//         Self{x, y}
//     }
// }



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
        .add_startup_system(setup)
        .add_system(keyboard_input)
        .add_system(movement)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    println!("{:?}", window.width());
    let player_position = Vec3::new(0.0, -window.height() / 2.0 + 20.0, 1.);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: player_position,
                // translation: Vec3::new(200.0, 200.0, 1.0),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..Default::default()
            },
            texture: asset_server.load("Angel_126x144.png"),
            ..Default::default()
        })
        .insert(Player)
        .insert(Velocity(Vec2::new(0.,0.)))
        .insert(Stat {
            speed: 20.,
        });
}

fn keyboard_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &Stat), With<Player>>,
) {
    let (mut player_velocity, stat) = query.get_single_mut().unwrap();
    player_velocity.0.x = 0.0;
    player_velocity.0.y = 0.0;
    if keyboard_input.pressed(KeyCode::Left) {
        player_velocity.0.x = -stat.speed
    }

    if keyboard_input.pressed(KeyCode::Right) {
        player_velocity.0.x = stat.speed;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        player_velocity.0.y = -stat.speed;
    }

    if keyboard_input.pressed(KeyCode::Up) {
        player_velocity.0.y = stat.speed;
    }
    
    player_velocity.0 = player_velocity.0.normalize();
    println!("{:?}", player_velocity);
    println!("{:?}", stat);
    
}

fn movement(mut query: Query<(&Velocity, &mut Transform)>) {
    let (velocity, mut transform) = query.get_single_mut().unwrap();
    transform.translation.x += velocity.0.x;
    transform.translation.y += velocity.0.y;
}
