use bevy::prelude::*;

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, hello_system);
    }
}

fn hello_system() {
    println!("Hello world!");
}
