use specs::{Component, VecStorage, NullStorage};
use std::fmt::Display;
use derive_display_from_debug::Display;

pub trait ActionStopper {}

#[derive(Component, Debug, Display)]
#[storage(VecStorage)]
pub enum Innocence {
    Innocent,
    Guilty,
}

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct Gun;

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct Undetectable;

impl ActionStopper for Undetectable{}

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct Uncoppable;

impl ActionStopper for Uncoppable{}

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct Untrackable;

impl ActionStopper for Untrackable{}

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct Saved;

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct Macho;

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct Blocked;

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct Breakthrough;

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct Visiting;
