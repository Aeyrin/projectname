use bevy::prelude::*;
use bevy_physimple::prelude::*;

// CONSTANTS
//const PLAYER_SPEED: f32 = 10.0;

// RESOURCES

pub struct PlayerStats {
    pub speed: f32,
    pub terminal: f32,
    pub friction: f32,
}
impl Default for PlayerStats {
    fn default() -> Self {
        Self{
            speed: 10000.0,
            terminal: 1000.0,
            friction: 1.4,
        }
    }
}
pub struct Gravity(Vec2);

// COMPONENTS

#[derive(Component)]
struct Player;

// FUNCTIONS

fn console(
    str: String,
) {
    println!("{:?}", str);
}

// SYSTEMS

fn move_player (
    mut query: Query<&mut Vel, With<Player>>,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    stats: Res<PlayerStats>,
) {
    if input.pressed(KeyCode::A) {
        for mut vel in query.iter_mut() {
            vel.0.x -= time.delta_seconds() * stats.speed;
        }
    }
    if input.pressed(KeyCode::D) {
        for mut vel in query.iter_mut() {
            vel.0.x += time.delta_seconds() * stats.speed;
        }
    }
    if input.pressed(KeyCode::Space) {
        for mut vel in query.iter_mut() {
            vel.0.y += time.delta_seconds() * stats.speed;
        }
    }
}

fn gravity (
    mut query: Query<&mut Vel>,
    time: Res<Time>,
    gravity: Res<Gravity>,
    stats: Res<PlayerStats>,
) {
    // Vel is a component that's part of the KinematicBundle, its the
    // velocity of entity. this equation will constantly apply
    // a downards push on the velocity, using delta to make sure
    // its consistent regardless of framerate
    for mut vel in query.iter_mut() {
        vel.0 += time.delta_seconds() * gravity.0;
        console(format!("{}", vel.0));
        // totally unrelated, create friction and apply terminal velocity
        vel.0.y = vel.0.y.clamp(-stats.terminal, stats.terminal,);
        vel.0.x = vel.0.x.clamp(-stats.terminal, stats.terminal,);

        // youre still left with an atomic amount of speed
        // its techincally better to create a speed coefficient and then
        // apply that directly and invert it if you wanna go backwards
        if vel.0.x != 0. {
            vel.0.x = vel.0.x / stats.friction;
        }
        /*if vel.0.y != 0. {
            vel.0.y = vel.0.y / stats.friction;
        }*/
        /*if vel.0 > 1000 {
            vel.0 /= time.delta_seconds() * 2.0;
        }*/
        
    }
}


// STARTUP SYSTEMS

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    
    // gravity
    commands.insert_resource(Gravity(Vec2::new(0.0,-5000.0)));

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
            shape: CollisionShape::Square(Square::size(Vec2::splat(50.0))),
            ..Default::default()
        })
        .insert(Player)
    ;

    // floor
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("../assets/spritesiguess/block.png"),
            transform: Transform::from_xyz(0., -100., 0.),
            ..Default::default()
        })
        .insert_bundle(StaticBundle {
            marker: StaticBody,
            shape: CollisionShape::Square(Square::size(Vec2::new(2525.0, 64.0))),
            coll_layer: CollisionLayer::default(),
        })
    ;

    // floor2
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("../assets/spritesiguess/block.png"),
            transform: Transform::from_xyz(60., -150., 0.),
            ..Default::default()
        })
        .insert_bundle(StaticBundle {
            marker: StaticBody,
            shape: CollisionShape::Square(Square::size(Vec2::new(64.0, 64.0))),
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
        //resources
        .insert_resource(WindowDescriptor {
            title: "poe the potato".to_string(),
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .init_resource::<PlayerStats>()
        //startup systems
        .add_startup_system(setup)
        //systems
        .add_system(move_player)
        .add_system(gravity)
        //run
        .run();
}