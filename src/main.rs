use bevy::prelude::*;
use cup_stack::CupStackPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // resolution: (720., 1000.).into(),
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(CupStackPlugin)
        .run();
}
