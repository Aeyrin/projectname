use bevy::prelude::*;
use bevy_physimple::prelude::*;

#[derive(Component)]
struct Player;

fn move_player (mut query: Query<(&Player, &mut Transform)>,  input: Res<Input<KeyCode>>) {
    let (_player, mut transform) = query.single_mut();
    if input.pressed(KeyCode::W) {
        transform.translation.y+=20.;
    }
    if input.pressed(KeyCode::A) {
        transform.translation.x-=20.;
    }
    if input.pressed(KeyCode::S) {
        transform.translation.y-=20.;
    }  
    if input.pressed(KeyCode::D) {
        transform.translation.x+=20.;
    }
    /*for player in query.iter ()   {
        if input.pressed(KeyCode::W) {
            player.1.translation.y += 100.;
        }  
    }*/
}

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