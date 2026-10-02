use bevy::{
    input::{InputSystem, touch::TouchPhase},
    prelude::*,
};

use ui::*;

mod ui;

pub struct FlowPlugin;

impl Plugin for FlowPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
        app.add_sub_state::<GameOverState>();
        app.insert_resource(AttemptCount(0));
        app.add_event::<GameplayAttempt>();
        app.add_plugins(FlowUiPlugin);
        app.add_systems(
            PreUpdate,
            start_gameplay
                .run_if(in_state(GameState::Welcome))
                .after(InputSystem),
        );
        app.add_systems(
            PostUpdate,
            handle_gameplay_attempt_event.run_if(in_state(GameState::Gameplay)),
        );
        app.add_systems(
            Update,
            tick_debounce_timer.run_if(resource_exists::<GameplayDebounceTimer>),
        );
        app.add_systems(OnEnter(GameState::Gameplay), remove_debounce_timer);
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Default, Hash)]
pub enum GameState {
    #[default]
    Welcome,
    Gameplay,
    TryAgain,
    GameOver,
}

#[derive(SubStates, Clone, PartialEq, Eq, Hash, Debug, Default)]
#[source(GameState = GameState::GameOver)]
enum GameOverState {
    #[default]
    Success,
    Fail,
}

#[derive(
    Resource, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Deref, DerefMut,
)]
pub struct AttemptCount(pub u8);

#[derive(Event, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum GameplayAttempt {
    Success,
    Failure,
}

#[derive(Resource)]
struct GameplayDebounceTimer(Timer);

fn start_gameplay_timer(mut commands: Commands) {
    commands.insert_resource(GameplayDebounceTimer(Timer::from_seconds(
        0.2,
        TimerMode::Once,
    )));
}

fn tick_debounce_timer(mut timer: ResMut<GameplayDebounceTimer>, time: Res<Time>) {
    timer.0.tick(time.delta());
}

fn remove_debounce_timer(mut commands: Commands) {
    commands.remove_resource::<GameplayDebounceTimer>();
}

fn start_gameplay(
    commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut touch_events: EventReader<TouchInput>,
    debounce_gameplay_timer: Option<Res<GameplayDebounceTimer>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if debounce_gameplay_timer.is_some_and(|timer| timer.0.finished()) {
        game_state.set(GameState::Gameplay);
    }
    if mouse_input.just_pressed(MouseButton::Left) {
        start_gameplay_timer(commands);
        return;
    }
    for touch in touch_events.read() {
        if matches!(touch, TouchInput { phase: TouchPhase::Ended, .. }) {
            // next_state.set(GameState::Gameplay);
            start_gameplay_timer(commands);
            return;
        }
    }
}

fn handle_gameplay_attempt_event(
    mut attempt_count: ResMut<AttemptCount>,
    mut event_r: EventReader<GameplayAttempt>,
    mut game_state: ResMut<NextState<GameState>>,
    mut game_over_state: ResMut<NextState<GameOverState>>,
) {
    for evt in event_r.read() {
        **attempt_count += 1;
        match evt {
            GameplayAttempt::Success => {
                game_state.set(GameState::GameOver);
                game_over_state.set(GameOverState::Success);
            }
            GameplayAttempt::Failure => match **attempt_count {
                0..3 => game_state.set(GameState::TryAgain),
                _ => {
                    game_state.set(GameState::GameOver);
                    game_over_state.set(GameOverState::Fail);
                }
            },
        };
    }
}
