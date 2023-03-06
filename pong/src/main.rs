#![allow(unused)]
use ball::BallPlugin;
use bevy::{
    app::AppExit, ecs::system::Command, input::keyboard, math::Vec3Swizzles, prelude::*,
    sprite::collide_aabb::collide,
};
use std::f32::consts::PI;
use border::BorderPlugin;
use components::{
    Ball, BallMovement, BallVelocity, Player, PlayerAI, SpeedUp, SpriteSize, Velocity, VelocityAI,
};
use player::PlayerPlugin;
use ai::AI;
mod ball;
mod border;
mod components;
mod player;
mod ai;

const PLAYER_SIZE: (f32, f32) = (20., 125.);
const BALL_SIZE: (f32, f32) = (20., 20.);
const MAX_BOUNCE_ANGLE: f32 = (5. * PI) / 18.;
const PLAYER_SPEED: f32 = 12.;
const MAX_SPEED_UP: f32 = 37.;
const INITAL_SPEED: f32 = 5.;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    InGame,
    Paused,
}
#[derive(Resource, Component)]
struct Score1 {
    score: usize,
}

#[derive(Resource, Component)]
struct Score2 {
    score: usize,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 1400.0,
                height: 700.0,
                title: "pong".to_string(),
                ..Default::default()
            },
            ..Default::default()
        }))
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(PlayerPlugin)
        .add_plugin(BallPlugin)
        .add_plugin(AI)
        .add_plugin(BorderPlugin)
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(Score1 { score: 0 })
        .insert_resource(Score2 { score: 0 })
        .add_startup_system(setup)
        .add_state(AppState::InGame)
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(player_control)
                .with_system(ai_movement)
                .with_system(ball_collision_system)
                .with_system(ball_movement)
                .with_system(update_score1)
                .with_system(update_score2)
                .with_system(exit_app)
                .with_system(pause),
        )
        .add_system_set(SystemSet::on_enter(AppState::Paused).with_system(pause))
        .add_system_set(
            SystemSet::on_update(AppState::Paused)
                .with_system(play)
                .with_system(exit_app),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font: asset_server.load("fonts/PixeloidSansBold-GOjpP.ttf"),
                font_size: 50.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                left: Val::Px(650.0),
                ..default()
            },
            ..default()
        }),
        Score1 { score: 0 },
    ));
    commands.spawn((
        TextBundle::from_section(
            "",
            TextStyle {
                font: asset_server.load("fonts/PixeloidSansBold-GOjpP.ttf"),
                font_size: 50.0,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::TOP_CENTER)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                right: Val::Px(646.0),
                ..default()
            },
            ..default()
        }),
        Score2 { score: 0 },
    ));
}

pub fn player_control(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &Transform), With<Player>>,
) {
    if let Ok((mut velocity, transform)) = query.get_single_mut() {
        let translation = &transform.translation;
        velocity.y = if keyboard.pressed(KeyCode::W) {
            if translation.y + 85. < 350. {
                PLAYER_SPEED
            } else {
                0.
            }
        } else if keyboard.pressed(KeyCode::S) {
            if translation.y - 85. > -350. {
                -PLAYER_SPEED
            } else {
                0.
            }
        } else {
            0.
        }
    }
}

fn ai_movement(
    mut commands: Commands, 
    mut aiquery: Query<(&VelocityAI, &mut Transform), Without<Ball>>, 
    ballquery: Query<(&Transform), With<Ball>>,
) {
    for (transform) in ballquery.iter(){
        let trans = &transform.translation;
        for (velocity, mut transform) in aiquery.iter_mut() {
            let translation = &mut transform.translation;
            if trans.x >= 0.{
                if translation.y + 50. < trans.y{
                    translation.y += 5.;
                }
                else if translation.y - 50.> trans.y{
                    translation.y -= 5.;
                }
                else{
                    translation.y += 0.;
                }    
            }
            else{
                if translation.y < -10.{
                    translation.y += 5.;
                }
                else if translation.y > 10.{
                    translation.y -= 5.;
                }
            }
        }

    }
}

fn ball_movement(
    mut commands: Commands,
    mut score: ResMut<Score1>,
    mut score2: ResMut<Score2>,
    mut query: Query<
        (
            Entity,
            &mut BallVelocity,
            &mut Transform,
            &BallMovement,
            &mut SpeedUp,
        ),
        With<Ball>,
    >,
) {
    for (entity, mut velocity, mut transform, ball_movement, mut speedup) in query.iter_mut() {
        let translation = &mut transform.translation;
        let speedup = &mut speedup.speed;
        translation.y += velocity.y;
        translation.x += velocity.x;

        if ball_movement.auto_despawn {
            if translation.x >= 900. {
                translation.y = 0.;
                translation.x = 0.;
                velocity.y = 0.;
                velocity.x = 5.;
                *speedup = INITAL_SPEED;
                score.score += 1;
            } else if translation.x <= -900. {
                translation.y = 0.;
                translation.x = 0.;
                velocity.y = 0.;
                velocity.x = -5.;
                *speedup = INITAL_SPEED;
                score2.score += 1;
            }
            if translation.y <= -345. {
                velocity.x * 2.;
                velocity.y = velocity.y * -1.;
                translation.y += velocity.y;
                translation.x += velocity.x;
            } else if translation.y >= 345. {
                velocity.x * 2.;
                velocity.y = velocity.y * -1.;
                translation.y += velocity.y;
                translation.x += velocity.x;
            }
        }
    }
}

fn ball_collision_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut ball_query: Query<
        (
            Entity,
            &mut BallVelocity,
            &mut Transform,
            &SpriteSize,
            &mut SpeedUp,
        ),
        With<Ball>,
    >,
    player_query: Query<(Entity, &Transform, &SpriteSize), Without<Ball>>,
) {
    for (ball_entity, mut velocity, mut ball_transform, ball_size, mut speedup) in
        ball_query.iter_mut()
    {
        for (player_entity, player_tf, player_size) in player_query.iter() {
            let ball_scale = Vec2::from(ball_transform.scale.xy());
            let player_scale = Vec2::from(player_tf.scale.xy());
            let collision = collide(
                ball_transform.translation,
                ball_size.0 * ball_scale + 1.,
                player_tf.translation,
                (player_size.0) * player_scale,
            );

            let translation = &mut ball_transform.translation;
            let speedup = &mut speedup.speed;
            let relative_intersect_y =
                (player_tf.translation.y + (PLAYER_SIZE.1 / 2.)) - translation.y;
            let normalized_relative_intersection_y = (relative_intersect_y / (PLAYER_SIZE.1 / 2.));
            let bounce_angle = normalized_relative_intersection_y * MAX_BOUNCE_ANGLE;
            if let Some(_) = collision {
                audio.play(asset_server.load("sounds/Tink.ogg"));
                if *speedup >= MAX_SPEED_UP {
                    *speedup * 1.;
                } else {
                    *speedup += 0.5;
                }
                if velocity.x <= 0. {
                    velocity.y = 5. * bounce_angle.sin();
                    velocity.x = 5. + (*speedup * bounce_angle.cos());
                    translation.y += velocity.y;
                    translation.x += velocity.x;
                } else if velocity.x >= 0. {
                    velocity.y = 5. * -bounce_angle.sin();
                    velocity.x = -5. + ((*speedup * -1.) * bounce_angle.cos());
                    translation.y += velocity.y;
                    translation.x += velocity.x;
                }
            }
        }
    }
}

fn update_score1(score: Res<Score1>, mut query: Query<&mut Text, With<Score1>>) {
    let mut text = query.single_mut();
    text.sections[0].value = score.score.to_string();
}
fn update_score2(score: Res<Score2>, mut query: Query<&mut Text, With<Score2>>) {
    let mut text = query.single_mut();
    text.sections[0].value = score.score.to_string();
}

fn play(mut keyboard: ResMut<Input<KeyCode>>, mut app_state: ResMut<State<AppState>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        app_state.pop().unwrap();
        keyboard.reset(KeyCode::Space);
    }
}

fn pause(
    mut commands: Commands,
    mut keyboard: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        app_state.push(AppState::Paused).unwrap();
        keyboard.reset(KeyCode::Space);
    }
}

fn exit_app(
    mut keyboard: ResMut<Input<KeyCode>>,
    mut exit: EventWriter<AppExit>,
    score1: Res<Score1>,
    score2: Res<Score2>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);

        let result = if score1.score > score2.score {
            "Player1 has won!"
        } else if score1.score == score2.score {
            "Draw"
        } else {
            "Player2 has won!"
        };

        println!("{}", result);
    }
}
