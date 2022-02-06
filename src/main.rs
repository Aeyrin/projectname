use bevy::prelude::*;
use bevy_physimple::prelude::*;

#[derive(Component)]
struct Player;

fn move_player (mut query: Query<(&Player, &mut Transform)>,  input: Res<Input<KeyCode>>) {
    let (_player, mut transform) = query.single_mut();
    if input.pressed(KeyCode::W) {
        transform.translation.y += 20.;
    }
    if input.pressed(KeyCode::A) {
        transform.translation.x -= 20.;
    }
    if input.pressed(KeyCode::S) {
        transform.translation.y -= 20.;
    }  
    if input.pressed(KeyCode::D) {
        transform.translation.x += 20.;
    }
    /*for player in query.iter ()   {
        if input.pressed(KeyCode::W) {
            player.1.translation.y += 100.;
        }  
    }*/
}

<<<<<<< HEAD
fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("Erin :D".to_string()));
    commands.spawn().insert(Person).insert(Name("Kairo :(".to_string()));
    commands.spawn().insert(Person).insert(Name("faint as well (gavian)".to_string()));
}

struct GreetTimer (Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished()   {
        for name in query.iter ()   {
            println! ("aaa eeeeee aaaaa e {}!", name.0);
        }
    }    
}

=======
>>>>>>> 7fbd7b05c39da58cec7b6d7e98e29e6725dd4ff9
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("../assets/spritesiguess/totato.png"),
        ..Default::default()
    }).insert(Player);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(Physics2dPlugin)
        .add_startup_system(setup)
        .add_system(move_player)
        .run();
}