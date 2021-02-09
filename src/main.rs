#![allow(dead_code)]
extern crate specs;

use specs::{World, WorldExt};
use specs::DispatcherBuilder;

use scumsim::entities::create_player;
use scumsim::components::*;
use scumsim::components::attributes::*;
use scumsim::systems::*;
use scumsim::resources::*;

fn main() {
    env_logger::init();

    let mut world = World::new();
    world.register::<Faction>();
    world.register::<Gun>();
    world.insert(CurrentNight(Night(0)));

    let mut dispatcher = DispatcherBuilder::new()
        .with(UpdateTargets, "update_targets", &[])
        .with(UpdateVisits, "update_visits", &["update_targets"])
        .with(BlockActions, "blockers", &["update_visits"])
        .with(InfoActions::<actions::Cop, Innocence, Uncoppable>::new(), "cops", &["blockers"])
        .with(InfoActions::<actions::Detective, Role, Undetectable>::new(), "detectives", &["blockers"])
        .with(InfoActions::<actions::Track, Position, Untrackable>::new(), "trackers", &["blockers"])
        //.with(TrackActions, "trackers", &["blockers"])
        .with(WatchActions, "watchers", &["blockers"])
        .with(SaveActions, "doctors", &["cops", "detectives", "trackers", "watchers"])
        .with(KillActions, "killers", &["doctors"])
        .with(PrintResults, "results", &["killers"])
        .with(RemoveEffects, "remove_effects", &["results"])
        .with(ProcessDeaths, "deaths", &["results"])
        .with(FinishNight, "advance_night", &["deaths"])
        .build();

    dispatcher.setup(&mut world);

    create_player(&mut world, String::from("player1"), Faction::Town, Role::Detective, vec![Modifier::Breakthrough]);
    create_player(&mut world, String::from("player2"), Faction::Mafia, Role::Godfather, vec![]);
    create_player(&mut world, String::from("player3"), Faction::Town, Role::Doctor, vec![]);
    create_player(&mut world, String::from("player4"), Faction::Mafia, Role::Roleblocker, vec![Modifier::Macho]);
    create_player(&mut world, String::from("player5"), Faction::Town, Role::Watcher, vec![]);
    create_player(&mut world, String::from("player6"), Faction::Town, Role::Tracker, vec![]);
    
    dispatcher.dispatch(&mut world);
    world.maintain();
    dispatcher.dispatch(&mut world);
    world.maintain();
}
