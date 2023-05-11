mod player;
mod camera;
mod map;

pub mod prelude {
    pub use bevy::prelude::*;
    pub use bevy_rapier2d::prelude::*;
    pub const HEIGHT: f32 = 480.0;
    pub const WIDTH: f32 = 640.0;

    pub use crate::camera::*;
    pub use crate::player::*;
    pub use crate::map::*;

    #[derive(Component, Deref, DerefMut)]
    pub struct AnimationTimer(pub Timer);

    #[derive(Component, Debug)]
    pub struct AnimationIndices {
        pub first: usize,
        pub last: usize,
    }
}
