use std::fmt;
use std::fmt::Display;
use derive_display_from_debug::Display;
use specs::{Component, Entity, VecStorage, NullStorage};
use super::resources::Night;
pub mod actions;
pub mod attributes;

/// The name of the player as other players can identify them
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Name(pub String);

#[derive(Component)]
#[storage(VecStorage)]
pub enum Faction {
    Town,
    Mafia,
}

/// Inner is the night died
#[derive(Component)]
#[storage(VecStorage)]
pub struct Dead(pub Night);

/// Signifies that player is dead and did not die tonight
#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct LongDead;

/// Current target for a night action
/// None if action does not take a target or no action being taken
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Target(pub Option<Entity>);

/// Current location, relative to another player, based on visiting night action
/// None if player 'went nowhere' on current night
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position(pub Option<Entity>);

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ent) = self.0 {
            write!(f, "{:?}", ent)
        } else {
            write!(f, "nowhere")
        }
    }
}

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
    Tracker,
    Watcher,
    Roleblocker,
    Doctor,
    Goon,
    Godfather,
}

#[derive(Clone, Debug, Display)]
pub enum Modifier {
    Breakthrough,
    Macho,
}

#[derive(Component, Clone, Debug, Display)]
#[storage(VecStorage)]
pub struct Modifiers(pub Vec<Modifier>);
