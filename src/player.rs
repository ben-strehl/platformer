use crate::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_player)
            .add_systems(
                (
                    apply_gravity,
                    player_control,
                    player_jump,
                    move_player,
                    )
                .chain()
                )
            .add_system(animate_player);
    }
}

#[derive(Component)]
struct PlayerVelocity(Vec2);

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let handle = asset_server.load("spritesheets/player-anim.png");
    let texture_atlas = 
        TextureAtlas::from_grid(handle, Vec2::new(33.0, 32.0), 14, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices { first: 0, last: 3 };

    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(0),
                transform: Transform::from_scale(Vec3::splat(1.0)),
                ..default()
            },
            RigidBody::KinematicPositionBased,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            animation_indices,
            Name::new("Player"),
        ))
        .insert(PlayerVelocity(Vec2::ZERO))
        .insert(Collider::cuboid(20.0 / 2.0, 33.0 / 2.0))
        .insert(KinematicCharacterController::default());
}

fn animate_player(
    mut query: Query<(&PlayerVelocity, &mut TextureAtlasSprite, &mut AnimationIndices)>,
) {
    let (velocity, mut sprite, mut indices) = query.get_single_mut().expect("Player not found in animate_player");
    //Make sprite index last so animate_sprites updates it to first
    if velocity.0.x == 0.0 && indices.first != 0 {
        indices.first = 0;
        indices.last = 3;
        sprite.index = 3;
    } else if velocity.0.x != 0.0 && indices.first != 4 {
        indices.first = 4;
        indices.last = 9;
        sprite.index = 9;
    }
    if velocity.0.x < 0.0 {
        sprite.flip_x = true;
    } else if velocity.0.x > 0.0 {
        sprite.flip_x = false;
    }
}

fn player_control(
    time: Res<Time>,
    mut player: Query<&mut PlayerVelocity>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut velocity = player
        .get_single_mut()
        .expect("No player found in player_control");
    if keyboard_input.pressed(KeyCode::A) {
        velocity.0 += Vec2::new(-20.0 * time.delta_seconds(), 0.0);
    } 
    if keyboard_input.pressed(KeyCode::D) {
        velocity.0 += Vec2::new(20.0 * time.delta_seconds(), 0.0);
    } 
    if !keyboard_input.pressed(KeyCode::A) && !keyboard_input.pressed(KeyCode::D) {
        velocity.0.x = 0.0;
    }
}

fn player_jump(
    time: Res<Time>,
    mut player: Query<(&mut PlayerVelocity, &KinematicCharacterControllerOutput)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let (mut velocity, controller) = player
        .get_single_mut()
        .expect("No player found in player_jump");

    if keyboard_input.pressed(KeyCode::Space) && controller.grounded {
        velocity.0 += Vec2::new(0.0, 50.0 * time.delta_seconds());
    }
}

fn move_player(
    time: Res<Time>, 
    mut controllers: Query<(&mut KinematicCharacterController, &PlayerVelocity)>
) {
    let (mut controller, velocity) = controllers.get_single_mut()
            .expect("Found multiple or no player in move_player");
    controller.translation = Some(velocity.0 * time.delta_seconds());
}

fn apply_gravity(
    mut player: Query<&mut PlayerVelocity>,
    time: Res<Time>,
) {
    let mut velocity = player
        .get_single_mut()
        .expect("Player not found in apply_gravity");

    velocity.0 += Vec2::new(0.0, -20.0 * time.delta_seconds());
}
