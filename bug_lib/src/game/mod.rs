use std::time::Duration;

use bevy::{
    math::{vec2, vec3},
    prelude::*,
    time::Stopwatch,
};

use crate::{
    asset_loading::AppAssets,
    combat::prelude::Health,
    enemy::{Enemy, EnemyInitData, EnemyList, EnemyPool},
    state::AppState,
    tower::Tower,
    ui::{MenuButtonAction, OnGameOverMenuScreen},
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
        .add_systems(
            OnTransition {
                from: AppState::MainMenu,
                to: AppState::InGame,
            },
            (
                setup_game_timer,
                apply_deferred,
                reset_game_timer,
                setup_game_timer_ui,
            )
                .chain(),
        )
        .add_systems(Update, game_over.run_if(in_state(AppState::InGame)))
        .add_systems(OnEnter(AppState::GameOver), setup_game_over)
        .add_systems(
            OnTransition {
                from: AppState::MainMenu,
                to: AppState::InGame,
            },
            (setup_game),
        )
        .add_systems(
            Update,
            difficulty_increases_with_time.run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            OnTransition {
                from: AppState::AssetsLoading,
                to: AppState::MainMenu,
            },
            spawn_background_image,
        )
        .add_systems(Update, rotate_background_image);
    }
}

#[derive(Resource)]
pub struct DifficultyConfig {
    pub modifier: f32,
    pub difficulty_increase_timer: Timer,
    pub enemies_per_spawn_batch: f32,
    pub difficulty_level: i32,
}

#[derive(Resource)]
pub struct GameTimer(pub Stopwatch);

#[derive(Component)]
pub struct GameTimerUi;

#[derive(Component)]
pub struct BackgroundImage;

// Does generic housekeeping stuff to set the game up
pub fn setup_game(mut commands: Commands, assets: Res<AppAssets>) {
    commands.insert_resource(DifficultyConfig {
        modifier: 1.0,
        difficulty_increase_timer: Timer::new(Duration::from_secs(60), TimerMode::Repeating),
        enemies_per_spawn_batch: 5.,
        difficulty_level: 0,
    });

    let mut starting_enemy_data = Vec::new();

    starting_enemy_data.push(EnemyInitData {
        sprite: assets.enemy1_sprite.clone_weak(),
        health_range: (50.0..100.0),
        speed_range: (50.0..75.0),
        required_difficulty: 0,
        ..Default::default()
    });

    starting_enemy_data.push(EnemyInitData {
        sprite: assets.enemy2_sprite.clone_weak(),
        health_range: (150.0..175.),
        speed_range: (25.0..35.0),
        required_difficulty: 1,
        ..Default::default()
    });

    commands.insert_resource(EnemyPool(starting_enemy_data))
}

pub fn difficulty_increases_with_time(
    mut difficulty_config: ResMut<DifficultyConfig>,
    time: Res<Time>,
) {
    difficulty_config
        .difficulty_increase_timer
        .tick(time.delta());

    if difficulty_config.difficulty_increase_timer.finished() {
        // Increase difficulty modifier
        difficulty_config.modifier += 0.1;

        // Add a new enemy to the pool
        difficulty_config.difficulty_level += 1;
    }
}

pub fn setup_game_timer(mut commands: Commands) {
    commands.insert_resource(GameTimer(Stopwatch::new()))
}

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
    assets: Res<AppAssets>,
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
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
            OnGameOverMenuScreen,
        ))
        .with_children(|parent| {
            parent.spawn((TextBundle::from_section(
                "Game Over",
                TextStyle {
                    font: font.clone_weak(),
                    font_size: 80.,
                    color: Color::RED,
                },
            ),));
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(250.),
                            height: Val::Px(65.),
                            margin: UiRect::all(Val::Px(20.)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..Default::default()
                    },
                    MenuButtonAction::BackToMainMenu,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Continue",
                        TextStyle {
                            font_size: 40.,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..Default::default()
                        },
                    ));
                });
        });
}

fn spawn_background_image(mut commands: Commands, assets: Res<AppAssets>) {
    commands.spawn((
        SpriteBundle {
            texture: assets.background_image.clone_weak(),
            transform: Transform::from_xyz(0., 0., -5.).with_scale(vec3(2., 2., 1.)),
            ..Default::default()
        },
        BackgroundImage,
    ));
}

fn rotate_background_image(mut query: Query<(&BackgroundImage, &mut Transform)>, time: Res<Time>) {
    if let Ok((_, mut t)) = query.get_single_mut() {
        t.rotate_z(time.delta_seconds().sin() * 0.01)
    }
}
