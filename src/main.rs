use bevy::prelude::*;
use platformer::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::input::common_conditions::input_toggle_active;

fn main() {
    App::new()
        .add_state::<GameState>()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (WIDTH, HEIGHT).into(),
                title: "Garbage Platformer".to_string(),
                resizable: false,
                ..default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest()))
        .add_plugin(CameraPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::I))
            )
        .add_system(animate_sprites.in_base_set(CoreSet::PostUpdate))
        .add_system(move_to_play.in_schedule(OnEnter(GameState::Menu)))
        .run();
}

fn animate_sprites(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlasSprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

fn move_to_play(
    mut next_state: ResMut<NextState<GameState>>,
) {
    next_state.set(GameState::Play);
}
