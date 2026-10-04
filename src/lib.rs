use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use camera::*;
use cup::*;
use flow::FlowPlugin;
use game::*;
use input::*;
use throwable::*;
use ui::*;

mod camera;
mod cup;
mod flow;
mod game;
mod input;
mod throwable;
mod ui;

/// The whole game, minus `DefaultPlugins` and the window config, which belong to the host.
pub struct CupStackPlugin;

impl Plugin for CupStackPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RapierPhysicsPlugin::<NoUserData>::default(),
            // RapierDebugRenderPlugin::default(),
        ))
        .add_plugins(FlowPlugin)
        .add_plugins((
            UiPlugin,
            GamePlugin,
            CameraPlugin,
            CupsPlugin,
            ThrowInputPlugin,
            ThrowablePlugin,
        ));
    }
}
