use crate::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(gen_map);
    }
}

fn gen_map(
    mut commands: Commands,
){
    commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, -140.0, 0.0),
                    scale: Vec3::new(640.0, 240.0, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgb(1.0, 1.0, 1.0),
                    ..default()
                },
                ..default()
            },
            ))
        .insert(Collider::cuboid(320.0, 120.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -140.0, 0.0)));
}
