#![allow(dead_code)]
extern crate specs;

use specs::{World, WorldExt};
use specs::DispatcherBuilder;

use scumsim::entities::create_player;
use scumsim::components::*;
use scumsim::components::actions::*;
use scumsim::components::attributes::*;
use scumsim::systems::*;

fn main() {
    env_logger::init();

    let mut world = World::new();
    world.register::<Faction>();
    world.register::<Gun>();
    world.register::<Kill>();
    world.register::<Modifier>();

    let mut dispatcher = DispatcherBuilder::new()
        .with(UpdateTargets, "update_targets", &[])
        .with(UpdateVisits, "update_visits", &["update_targets"])
        .with(BlockActions, "blockers", &["update_visits"])
        .with(CopActions, "cops", &["blockers"])
        .with(DetActions, "detectives", &["blockers"])
        .with(PrintResults, "results", &["cops", "detectives"])
        .build();

    dispatcher.setup(&mut world);

    create_player(&mut world, String::from("nastykast"), Faction::Town, Role::Cop, vec![Modifier::Breakthrough]);
    create_player(&mut world, String::from("Red123"), Faction::Mafia, Role::Godfather, vec![]);
    create_player(&mut world, String::from("BlueMarble"), Faction::Town, Role::Detective, vec![]);
    create_player(&mut world, String::from("TheFranswer"), Faction::Town, Role::Roleblocker, vec![]);
    
    dispatcher.dispatch(&mut world);
    world.maintain();
}
