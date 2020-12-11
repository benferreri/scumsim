#![allow(dead_code)]
#![allow(unused_imports)]
use specs::{Component, VecStorage, NullStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Cop;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Detective;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Block;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Kill;
