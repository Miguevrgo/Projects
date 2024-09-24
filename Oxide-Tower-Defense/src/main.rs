use bevy::prelude::*;
use crate::component::*;
mod component;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update,setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Position { x: 100.0, y: 100.0 },
        Health { value: 100 },
        Movement { speed: 5.0, direction: Vec2::new(1.0, 0.0) },
    ));

    commands.spawn((
        Position { x: 200.0, y: 200.0 },
        Health { value: 50 },
        Movement { speed: 3.0, direction: Vec2::new(-1.0, 0.0) },
        Attack { damage: 15, range: 50.0 },
        EnemyType { type: EnemyType::Orc },
    ));

    commands.spawn((
        Position { x: 300.0, y: 300.0 },
        Range { radius: 100.0 },
        Attack { damage: 25, range: 150.0 },
        TowerType { type: TowerType::Archer },
    ));
}
