use bevy::prelude::*;
use bevy_physimple::prelude::*;

// CONSTANTS
const PLAYER_SPEED: f32 = 20.0;

// RESOURCES

pub struct Gravity(Vec2);

// COMPONENTS

#[derive(Component)]
struct Player;



// SYSTEMS

fn move_player (mut query: Query<(&Player, &mut Transform)>,  input: Res<Input<KeyCode>>) {
    let (_player, mut transform) = query.single_mut();
    if input.pressed(KeyCode::A) {
        transform.translation.x -= PLAYER_SPEED;
    }
    if input.pressed(KeyCode::D) {
        transform.translation.x += PLAYER_SPEED;
    }
}

fn gravity (
    mut query: Query<&mut Vel>,
    time: Res<Time>,
    gravity: Res<Gravity>,
) {
    // Vel is a component that's part of the KinematicBundle, its the
    // velocity of entity. this equation will constantly apply
    // a downards push on the velocity, using delta to make sure
    // its consistent regardless of framerate
    for mut vel in query.iter_mut() {
        vel.0 += time.delta_seconds() * gravity.0;
    }
}

// STARTUP SYSTEMS

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    
    // gravity
    commands.insert_resource(Gravity(Vec2::new(0.0,-540.0)));

    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // player
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("../assets/spritesiguess/totato.png"),
            //transform: Transform {},
            ..Default::default()
        })
        .insert_bundle(KinematicBundle {
            shape: CollisionShape::Square(Square::size(Vec2::splat(200.0))),
            ..Default::default()
        })
        .insert(Player)
    ;

    // floor
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("../assets/spritesiguess/block.png"),
            transform: Transform::from_xyz(0., -400., 0.),
            ..Default::default()
        })
        .insert_bundle(StaticBundle {
            marker: StaticBody,
            shape: CollisionShape::Square(Square::size(Vec2::new(600.0, 600.0))),
            coll_layer: CollisionLayer::default(),
        })
    ;
}

// MAIN

fn main() {
    App::new()
        //plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(Physics2dPlugin)
        //startup systems
        .add_startup_system(setup)
        //systems
        .add_system(move_player)
        .add_system(gravity)
        //run
        .run();
}