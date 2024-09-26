use ::bevy::prelude::*;

#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Health {
    pub value: u16,
}

#[derive(Component)]
pub struct Money {
    pub value: u32,
}

#[derive(Component)]
pub struct Attack {
    pub damage: u32,
    pub range: f32,
}

#[derive(Component)]
pub struct Movement {
    pub speed: f32,
    pub direction: Vec2,
}

#[derive(Component)]
pub struct EnemyType {
    pub enemy: EnemyTypes,
}

#[derive(Component)]
pub struct TowerType {
    pub tower: TowerTypes,
}

#[derive(Component)]
pub struct Cell {
    pub x: u16,
    pub y: u16,
    pub used: bool,
}

pub enum EnemyTypes {
    Triangle,
    Square,
    Pentagon,
}

pub enum TowerTypes {
    Crab,
    Go,
    Snake,
}
