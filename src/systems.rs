use log::error;
use std::marker::PhantomData;
use specs::{Component, Read, Write, ReadStorage, WriteStorage, Entities, System};
use super::components::*;
use super::components::actions::Action;
use super::components::attributes::*;
use super::resources::*;

/// Update targets of all living players from input
pub struct UpdateTargets;
impl<'a> System<'a> for UpdateTargets {
    type SystemData = (Entities<'a>,
                       ReadStorage<'a, Name>,
                       ReadStorage<'a, Dead>,
                       ReadStorage<'a, Faction>,
                       WriteStorage<'a, Target>,
                       Read<'a, CurrentNight>);

    fn run(&mut self, (entities, names, dead, factions, mut targets, night): Self::SystemData) {
        use specs::Join;

        let new_target_name_maf = String::from(
            if night.0.0 > 0 {"nastykast"} else {"BlueMarble"}
        );

        let new_target_name_town = String::from(
            if night.0.0 > 0 {"TheFranswer"} else {"Red123"}
        );

        let new_target_maf = (&entities, &names, !&dead).join()
            .find(|(_, name, ())| name.0 == new_target_name_maf)
            .map(|(entity, _, ())| entity);

        let new_target_town = (&entities, &names, !&dead).join()
            .find(|(_, name, ())| name.0 == new_target_name_town)
            .map(|(entity, _, ())| entity);

        for (faction,target,()) in (&factions, &mut targets, !&dead).join() {
            if let Faction::Town = faction {
                target.0 = new_target_town;
            } else {
                target.0 = new_target_maf;
            }
        }
    }
}

/// Update the location of anyone with an active visiting action
pub struct UpdateVisits;
impl<'a> System<'a> for UpdateVisits {
    type SystemData = (Entities<'a>,
                       ReadStorage<'a, Visiting>,
                       ReadStorage<'a, Target>,
                       ReadStorage<'a, Blocked>,
                       WriteStorage<'a, Position>);

    fn run(&mut self, (entities, visiting, targets, blocked, mut positions): Self::SystemData) {
        use specs::Join;
        for (entity,target, _) in (&entities, &targets, &visiting).join() {
            let new_pos = if let Some(_) = blocked.get(entity) {
                Position(None)
            } else {
                Position(target.0)
            };
            let res = positions.insert(entity, new_pos);
            if let Err(e) = res {
                error!("error when updating position of {:?}: {:?}", entity, e);
            }
        }
    }
}

/// Process all block actions and give results as a `NightResult`
pub struct BlockActions;
impl<'a> System<'a> for BlockActions {
    type SystemData = (Entities<'a>,
                       ReadStorage<'a, actions::Block>,
                       ReadStorage<'a, Target>,
                       ReadStorage<'a, Breakthrough>,
                       WriteStorage<'a, Blocked>,
                       WriteStorage<'a, NightResult>);

    fn run(&mut self, data: Self::SystemData) {
        let (entities, blockers, targets, breakthroughs, mut blocked, mut results) = data;
        use specs::Join;

        for (entity, target, _) in (&entities, &targets, &blockers).join() {
            let success: bool;
            if let Some(target) = target.0 {
                if let Some(_) = breakthroughs.get(target) {
                    success = false;
                } else {
                    success = true;
                    let res = blocked.insert(target, attributes::Blocked);
                    if let Err(e) = res {
                        error!("error when {:?} is blocked: {:?}", target, e);
                    }
                }
            } else {
                success = false;
            }
            let res = results.insert(entity, NightResult {
                success,
                val: String::from("n/a"),
            });
            if let Err(e) = res {
                error!("error when {:?} gets block result: {:?}", entity, e);
            }
        }
    }
}

pub struct InfoActions<A, I, S> where
    A: Action + Component,
    I: Component + std::fmt::Display,
    S: ActionStopper + Component {
        _action:  PhantomData<A>,
        _info:    PhantomData<I>,
        _stopper: PhantomData<S>,
}

impl<A, I, S> InfoActions<A, I, S> where 
    A: Action + Component,
    I: Component + std::fmt::Display,
    S: ActionStopper + Component {

    pub fn new() -> InfoActions<A,I,S> {
        InfoActions {
            _action: PhantomData,
            _info: PhantomData,
            _stopper: PhantomData,
        }
    }
}

impl<'a, A, I, S> System<'a> for InfoActions<A, I, S> where
    A: Action + Component,
    I: Component + std::fmt::Display,
    S: ActionStopper + Component {
    type SystemData = (Entities<'a>,
                       ReadStorage<'a, A>,
                       ReadStorage<'a, Blocked>,
                       ReadStorage<'a, Target>,
                       ReadStorage<'a, I>,
                       ReadStorage<'a, S>,
                       WriteStorage<'a, NightResult>);

    fn run(&mut self, data: Self::SystemData) {
        let (entities, actions, blocked, targets, info_storage, stoppers, mut results) = data;
        use specs::Join;

        for (entity, target, action, blocked) in (&entities, &targets, &actions, (&blocked).maybe()).join() {
            // if cop is blocked or if there is no target, fail
            let (success, info) = match (action.active(), blocked, target.0) {
                (true, None,Some(target)) => {
                    if let None = stoppers.get(target) {
                        (true, info_storage.get(target).unwrap().to_string())
                    } else {
                        (false, String::from("n/a"))
                    }
                }
                (true, Some(_), _) | (true, _, None) | (false, _, _) => (false, String::from("n/a")),
            };
            let res = results.insert(entity, NightResult {
                success,
                val: info,
            });
            if let Err(e) = res {
                error!("error when {:?} gets result: {:?}", entity, e);
            }
        }
    }
}

/// Process all cop actions and give results as a `NightResult`
pub struct CopActions;
impl<'a> System<'a> for CopActions {
    type SystemData = (Entities<'a>,
                       ReadStorage<'a, actions::Cop>,
                       ReadStorage<'a, Blocked>,
                       ReadStorage<'a, Target>,
                       ReadStorage<'a, Innocence>,
                       WriteStorage<'a, NightResult>);

    fn run(&mut self, data: Self::SystemData) {
        let (entities, cops, blocked, targets, inno_storage, mut results) = data;
        use specs::Join;

        for (entity, target, cop, blocked) in (&entities, &targets, &cops, (&blocked).maybe()).join() {
            // if cop is blocked or if there is no target, fail
            let (success, inno) = match (cop.active(), blocked, target.0) {
                (true, None, Some(target)) => (true, inno_storage.get(target).unwrap().to_string()),
                (true, Some(_), _) | (true, _, None) | (false, _, _) => (false, String::from("n/a")),
            };
            let res = results.insert(entity, NightResult {
                success,
                val: inno,
            });
            if let Err(e) = res {
                error!("error when {:?} gets cop result: {:?}", entity, e);
            }
        }
    }
}

/// Process all detective actions and give results as a `NightResult`
pub struct DetActions;
impl<'a> System<'a> for DetActions {
    type SystemData = (Entities<'a>,
                       ReadStorage<'a, actions::Detective>,
                       ReadStorage<'a, Blocked>,
                       ReadStorage<'a, Target>,
                       ReadStorage<'a, Role>,
                       ReadStorage<'a, Undetectable>,
                       WriteStorage<'a, NightResult>);

    fn run(&mut self, data: Self::SystemData) {
        let (entities, dets, blocked, targets, 
             role_storage, detfails, mut results) = data;
        use specs::Join;

        for (entity, target, _, blocked) in (&entities, &targets, &dets, (&blocked).maybe()).join() {
            let (success,role) = match (blocked,target.0) {
                (None,Some(target)) => {
                    if let None = detfails.get(target) {
                        (true, role_storage.get(target).unwrap().to_string())
                    } else {
                        (false, String::from("n/a"))
                    }
                }
                (Some(_),_) | (_,None) => {
                    (false, String::from("n/a"))
                }
            };
            let res = results.insert(entity, NightResult {
                success,
                val: role,
            });
            if let Err(e) = res {
                error!("error when {:?} gets det result: {:?}", entity, e);
            }
        }
    }
}

/// Process all cop actions and give results as a `NightResult`
pub struct TrackActions;
impl<'a> System<'a> for TrackActions {
    type SystemData = (Entities<'a>,
                       ReadStorage<'a, actions::Track>,
                       ReadStorage<'a, Blocked>,
                       ReadStorage<'a, Target>,
                       ReadStorage<'a, Position>,
                       ReadStorage<'a, Name>,
                       WriteStorage<'a, NightResult>);

    fn run(&mut self, data: Self::SystemData) {
        let (entities, trackers, blocked, targets, positions, names, mut results) = data;
        use specs::Join;

        for (entity, target, _, blocked) in (&entities, &targets, &trackers, (&blocked).maybe()).join() {
            // if tracker is blocked or if there is no target, fail
            let (success, pos) = match (blocked,target.0) {
                (None,Some(target)) => {
                    if let Some(pos) = positions.get(target).unwrap().0 {
                        (true, names.get(pos).unwrap().0.clone())
                    } else {
                        (true, String::from("nowhere"))
                    }
                },
                (Some(_),_) | (_,None) => (false, String::from("n/a")),
            };
            let res = results.insert(entity, NightResult {
                success,
                val: pos,
            });
            if let Err(e) = res {
                error!("error when {:?} gets track result: {:?}", entity, e);
            }
        }
    }
}

/// Process all watch actions and give results as a `NightResult`
pub struct WatchActions;
impl<'a> System<'a> for WatchActions {
    type SystemData = (Entities<'a>,
                       ReadStorage<'a, actions::Watch>,
                       ReadStorage<'a, Blocked>,
                       ReadStorage<'a, Target>,
                       ReadStorage<'a, Position>,
                       ReadStorage<'a, Name>,
                       WriteStorage<'a, NightResult>);

    fn run(&mut self, data: Self::SystemData) {
        let (entities, watchers, blocked, targets, positions, names, mut results) = data;
        use specs::Join;

        for (entity, target, _, blocked) in (&entities, &targets, &watchers, (&blocked).maybe()).join() {
            // if watcher is blocked or if there is no target, fail
            let (success, visitors) = match (blocked,target.0) {
                (None,Some(target)) => {
                    let visitors = (&names, &positions).join()
                        .filter(|(_, pos)| if let Some(pos) = pos.0 {pos == target} else {false})
                        .map(|(name, _)| name.0.clone())
                        .collect::<Vec<String>>()
                        .join(", ");
                    (true, visitors)
                },
                (Some(_),_) | (_,None) => (false, String::from("n/a")),
            };
            let res = results.insert(entity, NightResult {
                success,
                val: visitors,
            });
            if let Err(e) = res {
                error!("error when {:?} gets cop result: {:?}", entity, e);
            }
        }
    }
}

/// Process all save actions and give results as a `NightResult`
pub struct SaveActions;
impl<'a> System<'a> for SaveActions {
    type SystemData = (Entities<'a>,
                       ReadStorage<'a, actions::Save>,
                       ReadStorage<'a, Blocked>,
                       ReadStorage<'a, Target>,
                       ReadStorage<'a, Macho>,
                       WriteStorage<'a, Saved>,
                       WriteStorage<'a, NightResult>);

    fn run(&mut self, data: Self::SystemData) {
        let (entities, doctors, blocked, targets, macho, mut saved, mut results) = data;
        use specs::Join;

        for (entity, target, _, blocked) in (&entities, &targets, &doctors, (&blocked).maybe()).join() {
            let success = match (blocked,target.0) {
                (None,Some(target)) => {
                    if let None = macho.get(target) {
                        let res = saved.insert(target, Saved);
                        if let Err(e) = res {
                            error!("error when {:?} is saved: {:?}", target, e);
                        }
                        true
                    } else {
                        false
                    }
                }
                (Some(_),_) | (_,None) => false,
            };
            let res = results.insert(entity, NightResult {
                success,
                val: String::from("n/a"),
            });
            if let Err(e) = res {
                error!("error when {:?} gets save result: {:?}", entity, e);
            }
        }
    }
}

/// Process all kill actions and give results as a `NightResult`
pub struct KillActions;
impl<'a> System<'a> for KillActions {
    type SystemData = (Entities<'a>,
                       Read<'a, CurrentNight>,
                       ReadStorage<'a, actions::Kill>,
                       ReadStorage<'a, Target>,
                       ReadStorage<'a, Saved>,
                       WriteStorage<'a, Dead>,
                       WriteStorage<'a, NightResult>);

    fn run(&mut self, data: Self::SystemData) {
        let (entities, night, killers, targets, doctored, mut dead, mut results) = data;
        use specs::Join;

        for (entity, target, _) in (&entities, &targets, &killers).join() {
            let success = match (night.0.0, target.0) {
                (0, _) | (_, None) => false,
                (_, Some(target)) =>  {
                    if let Some(_) = doctored.get(target) {
                        false
                    } else {
                        let res = dead.insert(target, Dead(night.0.clone()));
                        if let Err(e) = res {
                            error!("error when {:?} is killed: {:?}", target, e);
                        }
                        true
                    }
                },
            };
            let res = results.insert(entity, NightResult {
                success,
                val: String::from("n/a"),
            });
            if let Err(e) = res {
                error!("error when {:?} gets kill result: {:?}", entity, e);
            }
        }
    }
}

/// Print night results
pub struct PrintResults;
impl<'a> System<'a> for PrintResults {
    type SystemData = (Read<'a, CurrentNight>,
                       ReadStorage<'a, Name>,
                       ReadStorage<'a, Modifiers>,
                       ReadStorage<'a, Role>,
                       ReadStorage<'a, Target>,
                       ReadStorage<'a, NightResult>,
                       ReadStorage<'a, Dead>,
                       ReadStorage<'a, LongDead>);

    fn run(&mut self, data : Self::SystemData) {
        let (night, names, modifiers, roles, targets, results, dead, longdead) = data;
        use specs::Join;

        println!("Night {} results:", night.0.0);
        for (name, modifier, role, target, result, dead, ()) in 
            (&names, &modifiers, &roles, &targets, &results, (&dead).maybe(), !&longdead).join() {
                let target_name = if let Some(ent) = target.0 {
                    names.get(ent).unwrap().0.clone()
                } else {
                    String::from("nobody")
                };
                let mut modifier = modifier.0
                    .iter()
                    .map(|modifier|modifier.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");
                if modifier.len() > 0 {
                    modifier.push(' ');
                }
                println!("{}{} {} targets {} - {} - {}", modifier, role, name.0, target_name, if result.success { "success" } else { "fail" }, result.val);
                if let Some(_) = dead {
                    println!("{} {} died", role, name.0);
                }
            }
    }
}

/// Remove Blocked, Saved, etc. component from anyone who received it tonight
pub struct RemoveEffects;
impl<'a> System<'a> for RemoveEffects {
    type SystemData = (Entities<'a>,
                       WriteStorage<'a, Blocked>,
                       WriteStorage<'a, Saved>);

    fn run(&mut self, (entities, mut blocked, mut saved): Self::SystemData) {
        use specs::Join;

        for entity in (&entities).join() {
            if let Some(_) = blocked.get(entity) {
                blocked.remove(entity);
            }
            if let Some(_) = saved.get(entity) {
                saved.remove(entity);
            }
        }
    }
}

/// Make the dead people (`Dead`) `LongDead` and set `Target`s of dead people to `None`
pub struct ProcessDeaths;
impl<'a> System<'a> for ProcessDeaths {
    type SystemData = (Entities<'a>,
                       ReadStorage<'a, Dead>,
                       WriteStorage<'a, LongDead>,
                       WriteStorage<'a, Target>);

    fn run(&mut self, (entities, dead, mut longdead, mut targets): Self::SystemData) {
        use specs::Join;

        for (entity, _) in (&entities, &dead).join() {
            if let None = longdead.get(entity) {
                if let Err(e) = longdead.insert(entity, LongDead) {
                    error!("error when making {:?} LongDead: {:?}", entity, e);
                }
                if let Err(e) = targets.insert(entity, Target(None)) {
                    error!("error setting target of {:?} to None: {:?}", entity, e);
                }
            }
        }
    }
}

/// Advance to the next Night
pub struct FinishNight;
impl <'a> System<'a> for FinishNight {
    type SystemData = Write<'a, CurrentNight>;

    fn run(&mut self, mut night: Self::SystemData) {
        night.0 = Night(night.0.0 + 1);
    }
}
