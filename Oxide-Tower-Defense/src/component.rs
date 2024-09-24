use ::bevy::prelude::*;

#[derive(Component)]
pub struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
pub struct Health {
    value: u16,
}

#[derive(Component)]
pub struct Money {
    value: u32,
}

#[derive(Component)]
pub struct Attack {
    damage: u32,
    range: f32,
}

#[derive(Component)]
pub struct Movement {
    speed: f32,
    direction: Vec2,
}

pub enum EnemyType {
    Triangle,
    Square,
    Pentagon,
}

pub enum TowerType {
    Crab,
    Go,
    Snake,
}
