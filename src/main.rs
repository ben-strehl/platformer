mod camera;

use bevy::prelude::*;
use camera::CameraPlugin;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2,0.2,0.2)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WIDTH,
                height: HEIGHT,
                title: "Garbage Platformer".to_string(),
                resizable: false,
                ..default()
            },
            ..default()
        }))
        .run();
}
