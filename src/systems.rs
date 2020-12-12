use log::error;
use specs::{Read,ReadStorage, WriteStorage, Entities, System};
use super::components::*;
use super::components::attributes::*;
use super::resources::*;

/// Update targets of all living players from input
pub struct UpdateTargets;
impl<'a> System<'a> for UpdateTargets {
    type SystemData = (Entities<'a>,
                       ReadStorage<'a, Name>,
                       ReadStorage<'a, Dead>,
                       WriteStorage<'a, Target>);

    fn run(&mut self, (entities, names, dead, mut targets): Self::SystemData) {
        use specs::Join;

        let new_target_name = String::from("nastykast");

        let new_target = (&entities, &names, !&dead).join()
            .find(|(_, name, ())| name.0 == new_target_name)
            .map(|(entity, _, ())| entity);

        for (target,()) in (&mut targets, !&dead).join() {
            target.0 = new_target;
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

        for (entity, target, _, blocked) in (&entities, &targets, &cops, (&blocked).maybe()).join() {
            // if cop is blocked or if there is no target, fail
            let (success, inno) = match (blocked,target.0) {
                (None,Some(target)) => (true, inno_storage.get(target).unwrap().to_string()),
                (Some(_),_) | (_,None) => (false, String::from("n/a")),
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
            let success: bool;
            if let Some(target) = target.0 {
                if let Some(_) = doctored.get(target) {
                    success = false;
                } else {
                    success = true;
                    let res = dead.insert(target, Dead(night.0.clone()));
                    if let Err(e) = res {
                        error!("error when {:?} is killed: {:?}", target, e);
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
                error!("error when {:?} gets kill result: {:?}", entity, e);
            }
        }
    }
}

/// Print night results
pub struct PrintResults;
impl<'a> System<'a> for PrintResults {
    type SystemData = (ReadStorage<'a, Name>,
                       ReadStorage<'a, Role>,
                       ReadStorage<'a, Target>,
                       ReadStorage<'a, NightResult>,
                       ReadStorage<'a, Dead>,
                       ReadStorage<'a, LongDead>);

    fn run(&mut self, (names, roles, targets, results, dead, longdead): Self::SystemData) {
        use specs::Join;

        for (name, role, target, result, dead, ()) in 
            (&names, &roles, &targets, &results, (&dead).maybe(), !&longdead).join() {
                let target_name: String;
                if let Some(ent) = target.0 {
                    target_name = names.get(ent).unwrap().0.clone();
                } else {
                    target_name = String::from("nobody");
                }
                println!("{} {} targets {} - {} - {}", role, name.0, target_name, if result.success { "success" } else { "fail" }, result.val);
                if let Some(_) = dead {
                    println!("{} {} died", role, name.0);
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
