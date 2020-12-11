use specs::{Component, VecStorage, NullStorage};
use std::fmt::Display;
use derive_display_from_debug::Display;

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

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct Blocked;

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct Breakthrough;

#[derive(Component, Default, Debug)]
#[storage(NullStorage)]
pub struct Visiting;
