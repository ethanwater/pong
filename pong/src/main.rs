#![allow(unused)]
use std::f32::consts::PI;
use ball::BallPlugin;
use bevy::{prelude::*, ecs::system::Command, math::Vec3Swizzles, sprite::collide_aabb::collide};
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use border::BorderPlugin;
use components::{SpriteSize, Ball, Velocity, BallVelocity, Movement, Player, Border, SpeedUp};
use player::PlayerPlugin;
use player2::PlayerPlugin2;
mod player;
mod player2;
mod components;
mod ball;
mod border;

const PLAYER_SIZE: (f32, f32) = (20.,150.);
const BALL_SIZE: (f32, f32) = (20., 20.);
const BLACK: (f32, f32, f32) = (0.,0.,0.);
const WHITE: (f32, f32, f32) = (255.,255.,255.);
const MAX_BOUNCE_ANGLE: f32 = (5.*PI)/18.;
const PLAYER_SPEED: f32 = 12.;
const MAX_SPEED: f32 = 20.;
const INITAL_SPEED: f32 = 3.;
#[derive(Resource)]
pub struct WindowSize {
    pub w: f32,
    pub h: f32,
}

fn main() {
  App::new()
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor { 
            width: 1400.0,
            height:700.0,
            title: "pong".to_string(),
            ..Default::default()
        },
        ..Default::default()
    }))
    // .add_plugin(LogDiagnosticsPlugin::default())
    // .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_startup_system(setup)
    .add_plugin(PlayerPlugin)
    .add_plugin(PlayerPlugin2)
    .add_plugin(BallPlugin)
    .add_plugin(BorderPlugin)
    .add_system(ball_collision_system)
    .add_system(movement)
    .run();
}

fn setup(mut commands: Commands, mut window: ResMut<Windows>) {
    commands.spawn(Camera2dBundle::default());

    let window = window.get_primary_mut().unwrap();
    let (window_height, window_width) = (window.width(), window.height());

    let window_size = WindowSize{w: window_width, h: window_height};
    commands.insert_resource(window_size);
} 

fn movement(
    mut commands: Commands,
    win_size: Res<WindowSize>,
    mut query: Query<(Entity, &mut BallVelocity, &mut Transform, &Movement, &mut SpeedUp)>
) {
    for (entity, mut velocity, mut transform, movement, mut speedup) in query.iter_mut(){
        let translation = &mut transform.translation;
        let speedup = &mut speedup.speed;
        translation.y += velocity.y;
        translation.x += velocity.x;

        if movement.auto_despawn {
            //CURSED CODE!!!- LITERALLY CHAOS, KEEPING IT FOR HUMOR PURPOSES >:)
            //
            // if translation.x >= 700. || translation.x <= -700.{
            //     commands.entity(entity).despawn();
                
            //     commands.spawn(SpriteBundle {
            //         sprite: Sprite { 
            //             color: Color::rgb(WHITE.0,WHITE.1,WHITE.2),
            //             custom_size: Some(Vec2::new(BALL_SIZE.0, BALL_SIZE.1)),
            //             ..Default::default()
            //         },
            //         ..Default::default()
            //     })
            //     .insert(SpriteSize::from(BALL_SIZE))
            //     .insert(Movement {auto_despawn: true})
            //     .insert(SpeedUp{speed: INITAL_SPEED})
            //     .insert(Ball)
            //     .insert(BallVelocity {x: 5., y: 0.});

            if translation.x >= 700.{
                translation.y = 0.;
                translation.x =0.;
                velocity.y = 0.;
                velocity.x = 5.;
                *speedup = INITAL_SPEED;
                // commands.entity(entity).despawn();
                
            }
            if translation.x <= -700.{
                // commands.entity(entity).despawn();
                
                translation.y = 0.;
                translation.x =0.;
                velocity.y =0.;
                velocity.x = -5.;
                *speedup = INITAL_SPEED;
            }
            if translation.y <= -345. {
                velocity.x *2.;
                let relative_intersect_y = (-345. +(1400./2.)) - translation.y;
                let normalized_relative_intersection_y = (relative_intersect_y/(1400./2.));
                let bounce_angle = normalized_relative_intersection_y * MAX_BOUNCE_ANGLE;
                velocity.y = velocity.y * -1.;
                translation.y += velocity.y;
                translation.x += velocity.x;
            }
            else if translation.y >= 345. {
                velocity.x *2.;
                let relative_intersect_y = (345. +(1400./2.)) - translation.y;
                let normalized_relative_intersection_y = (relative_intersect_y/(1400./2.));
                let bounce_angle = normalized_relative_intersection_y * MAX_BOUNCE_ANGLE;
                velocity.y = velocity.y * -1.;
                translation.y += velocity.y;
                translation.x += velocity.x;
            }
        }
    }
}
fn ball_collision_system(
    mut commands: Commands,
    mut ball_query: Query<(Entity,&mut BallVelocity, &mut Transform, &SpriteSize, &mut SpeedUp), With<Ball>>,
    player_query: Query<(Entity, &Transform, &SpriteSize), Without<Ball>>,
) {
    for (ball_entity, mut velocity, mut ball_transform, ball_size, mut speedup) in ball_query.iter_mut(){
        for (player_entity, player_tf, player_size) in player_query.iter() {
            let ball_scale = Vec2::from(ball_transform.scale.xy());
            let player_scale = Vec2::from(player_tf.scale.xy());
            let collision = collide(
                ball_transform.translation,
                ball_size.0 * ball_scale + 1.,
                player_tf.translation,
                (player_size.0 + 10.) * player_scale
            );

            let translation = &mut ball_transform.translation;
            let speedup = &mut speedup.speed;
            let relative_intersect_y = (player_tf.translation.y +(PLAYER_SIZE.1/2.)) - translation.y;
            let normalized_relative_intersection_y = (relative_intersect_y/(PLAYER_SIZE.1/2.));
            let bounce_angle = normalized_relative_intersection_y * MAX_BOUNCE_ANGLE;
            if let Some(_) = collision {
                if *speedup >= MAX_SPEED {
                    *speedup * 1.;
                    // println!("{}", speedup);
                }
                else{
                    *speedup += 0.25;
                    // println!("{}", speedup);
                }
                if velocity.x <= 0.{
                    velocity.y = 5. * bounce_angle.sin();
                    velocity.x = 5. + (*speedup * bounce_angle.cos());
                    translation.y += velocity.y;
                    translation.x += velocity.x;
                    println!("{}", speedup);
                }
                else if velocity.x >= 0.{
                    velocity.y = 5. *- bounce_angle.sin();
                    velocity.x = -5. + ((*speedup*-1.) * bounce_angle.cos());
                    translation.y += velocity.y;
                    translation.x += velocity.x;
                    println!("{}", speedup);
                }
            }
        }
    }
}