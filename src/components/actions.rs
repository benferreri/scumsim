#![allow(dead_code)]
#![allow(unused_imports)]
use specs::{Component, VecStorage, NullStorage, Entity};

pub trait Action {
    fn new() -> Self;
    fn active(&self) -> bool;
    fn target(&self) -> &Option<Entity>;
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Cop {
    active: bool,
    target: Option<Entity>,
}

impl Action for Cop {
    fn new() -> Cop {
        Cop { active: false, target: None }
    }
    fn active(&self) -> bool {
        self.active
    }
    fn target(&self) -> &Option<Entity> {
        &self.target
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Detective {
    active: bool,
    target: Option<Entity>,
}

impl Action for Detective {
    fn new() -> Detective {
        Detective { active: false, target: None }
    }
    fn active(&self) -> bool {
        self.active
    }
    fn target(&self) -> &Option<Entity> {
        &self.target
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Track {
    active: bool,
    target: Option<Entity>,
}

impl Action for Track {
    fn new() -> Track {
        Track { active: false, target: None }
    }
    fn active(&self) -> bool {
        self.active
    }
    fn target(&self) -> &Option<Entity> {
        &self.target
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Watch {
    active: bool,
    target: Option<Entity>,
}

impl Action for Watch {
    fn new() -> Watch {
        Watch { active: false, target: None }
    }
    fn active(&self) -> bool {
        self.active
    }
    fn target(&self) -> &Option<Entity> {
        &self.target
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Block {
    active: bool,
    target: Option<Entity>,
}

impl Action for Block {
    fn new() -> Block {
        Block { active: false, target: None }
    }
    fn active(&self) -> bool {
        self.active
    }
    fn target(&self) -> &Option<Entity> {
        &self.target
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Save {
    active: bool,
    target: Option<Entity>,
}

impl Action for Save {
    fn new() -> Save {
        Save { active: false, target: None }
    }
    fn active(&self) -> bool {
        self.active
    }
    fn target(&self) -> &Option<Entity> {
        &self.target
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Kill {
    active: bool,
    target: Option<Entity>,
}

impl Action for Kill {
    fn new() -> Kill {
        Kill { active: false, target: None }
    }
    fn active(&self) -> bool {
        self.active
    }
    fn target(&self) -> &Option<Entity> {
        &self.target
    }
}
