use bevy::prelude::*;
mod components;
mod resources;
mod systems;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}
