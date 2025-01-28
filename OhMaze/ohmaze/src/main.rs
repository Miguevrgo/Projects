use bevy::prelude::*;
mod components;
mod entities;
mod systems;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}
