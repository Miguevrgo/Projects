use bevy::prelude::*;
mod components;
mod systems;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}
