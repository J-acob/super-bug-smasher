use bevy::{prelude::*, time::Stopwatch};

use crate::{
    asset_loading::AppAssets, combat::prelude::Health, enemy::Enemy, state::AppState, tower::Tower, ui::MenuButtonAction,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (game_timer_tick, update_game_timer_ui)
                .chain()
                .distributive_run_if(in_state(AppState::InGame)),
        )
        .insert_resource(GameTimer(Stopwatch::new()))
        .add_systems(
            OnTransition {
                from: AppState::MainMenu,
                to: AppState::InGame,
            },
            (reset_game_timer, setup_game_timer_ui),
        )
        .add_systems(Update, game_over.run_if(in_state(AppState::InGame)))
        .add_systems(OnTransition {
            from: AppState::InGame,
            to: AppState::GameOver,
        }, setup_game_over)
        ;
    }
}

#[derive(Resource)]
pub struct GameTimer(pub Stopwatch);

#[derive(Component)]
pub struct GameTimerUi;

pub fn game_timer_tick(mut game_timer: ResMut<GameTimer>, time: Res<Time>) {
    game_timer.0.tick(time.delta());
}

pub fn reset_game_timer(mut game_timer: ResMut<GameTimer>) {
    game_timer.0.reset();
}

pub fn setup_game_timer_ui(mut commands: Commands, assets: Res<AppAssets>) {
    let font = &assets.font;
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(25.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Game Timer",
                    TextStyle {
                        font: font.clone_weak(),
                        font_size: 40.,
                        color: Color::WHITE,
                    },
                ),
                GameTimerUi,
            ));
        });
}

pub fn update_game_timer_ui(
    game_timer: Res<GameTimer>,
    mut text_query: Query<&mut Text, With<GameTimerUi>>,
) {
    let mut text = text_query.single_mut();

    let timer = &game_timer.0;
    text.sections[0].value = format!(
        "{:02} : {:02}",
        timer.elapsed().as_secs() / 60,
        timer.elapsed().as_secs() % 60
    )
}

pub fn game_over(tq: Query<(&Tower, &Health)>, mut state: ResMut<NextState<AppState>>) {
    let (_, hp) = tq.single();

    if hp.0 <= 0. {
        state.set(AppState::GameOver)
    }
}

/// Displays stuff once the game is over and removes all entities that we don't need/care about
pub fn setup_game_over(
    mut commands: Commands,
    eq: Query<Entity, With<Enemy>>,
    tq: Query<Entity, With<Tower>>,
    gtq: Query<Entity, With<GameTimerUi>>,
    assets: Res<AppAssets>
) {
    for e in eq.iter() {
        commands.entity(e).despawn_recursive();
    }

    for e in tq.iter() {
        commands.entity(e).despawn_recursive();
    }

    for e in gtq.iter() {
        commands.entity(e).despawn_recursive();
    }
    
    let font = &assets.font;
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "Game Over",
                    TextStyle {
                        font: font.clone_weak(),
                        font_size: 80.,
                        color: Color::RED,
                    },
                ),
            ));
        });
}

