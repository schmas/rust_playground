use bevy::prelude::*;
use bevy_spaceship_game::{
    camera::CameraPlugin, debug::DebugPlugin, movement::MovementPlugin, spaceship::SpaceshipPlugin,
};

fn main() {
    App::new()
        // Bevy built-ins.
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 2500.0,
        })
        .add_plugins(DefaultPlugins)
        // User defined plugins.
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(CameraPlugin)
        .run();
}
