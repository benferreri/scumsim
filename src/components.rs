use specs::{Component, Entity, VecStorage};
use std::fmt::Display;
use derive_display_from_debug::Display;
pub mod actions;
pub mod attributes;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Name(pub String);

#[derive(Component)]
#[storage(VecStorage)]
pub enum Faction {
    Town,
    Mafia,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Target(pub Option<Entity>);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position(pub Option<Entity>);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct NightResult {
    pub success: bool,
    pub val: String,
}

#[derive(Component, Clone, Debug, Display)]
#[storage(VecStorage)]
pub enum Role {
    Vanilla,
    Cop,
    Sheriff,
    Detective,
    Roleblocker,
    Goon,
    Godfather,
}
