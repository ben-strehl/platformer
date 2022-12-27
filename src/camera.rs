use bevy::prelude::*;
use crate::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App){
        
    }
}

fn spawn_camera(commands: &mut Commands) {
    commands
        .spawn()
}