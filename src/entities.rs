use specs::{World,WorldExt,Entity,EntityBuilder,Builder};
use super::components::{Name,Faction,Target,Position,Role,Modifier,Modifiers,actions,actions::Action,attributes};

// certain roles will overwrite the faction
// e.g. if trying to make a Town Goon, a Mafia Goon will instead be returned
pub fn create_player(world: &mut World, name: String, faction: Faction, role: Role, modifiers: Vec<Modifier>) -> Entity {
    let player = world.create_entity()
        .base_player(name)
        .faction(faction);
    let mut player = give_role(player, role);
    player = give_modifiers(player, modifiers);
    player.build()
}

pub fn give_role<'a>(player: EntityBuilder<'a>, role: Role) -> EntityBuilder<'a> {
    let player = player.with(role.clone());
    match role {
        Role::Vanilla     => player.vanilla(),
        Role::Cop         => player.cop(),
        Role::Sheriff     => player.cop().detective(),
        Role::Detective   => player.detective(),
        Role::Tracker     => player.tracker(),
        Role::Watcher     => player.watcher(),
        Role::Roleblocker => player.roleblocker(),
        Role::Doctor      => player.doctor(),
        Role::Goon        => player.goon(),
        Role::Godfather   => player.godfather(),
    }
}

pub fn give_modifiers<'a>(player: EntityBuilder<'a>, modifiers: Vec<Modifier>) -> EntityBuilder<'a> {
    let mut player_upd = player;
    for modifier in modifiers.iter() {
        player_upd = match modifier {
            Modifier::Breakthrough => player_upd.breakthrough(),
            Modifier::Macho        => player_upd.macho(),
        };
    }
    player_upd.with(Modifiers(modifiers))
}

trait PlayerBuilder {
    fn base_player(self, name: String) -> Self;
    fn faction(self, faction: Faction) -> Self;
}

impl<'a> PlayerBuilder for EntityBuilder<'a> {

    fn base_player(self, name: String) -> Self {
        self
            .with(Name(name))
            .with(Position(None))
            .with(Target(None))
    }

    fn faction(self, faction: Faction) -> Self {
        let inno = match faction {
            Faction::Town  => attributes::Innocence::Innocent,
            Faction::Mafia => attributes::Innocence::Guilty,
        };
        self
            .with(faction)
            .with(inno)
    }
}

trait RoleBuilder {
    fn vanilla(self) -> Self;
    fn cop(self) -> Self;
    fn detective(self) -> Self;
    fn tracker(self) -> Self;
    fn watcher(self) -> Self;
    fn roleblocker(self) -> Self;
    fn doctor(self) -> Self;
    fn goon(self) -> Self;
    fn godfather(self) -> Self;
}

impl<'a> RoleBuilder for EntityBuilder<'a> {

    fn vanilla(self) -> Self {
        self
    }

    fn cop(self) -> Self {
        self
            .with(attributes::Visiting)
            .with(attributes::Gun)
            .with(actions::Cop::new())
    }

    fn detective(self) -> Self {
        self
            .with(attributes::Visiting)
            .with(attributes::Gun)
            .with(actions::Detective::new())
    }

    fn tracker(self) -> Self {
        self
            .with(attributes::Visiting)
            .with(actions::Track::new())
    }

    fn watcher(self) -> Self {
        self
            .with(attributes::Visiting)
            .with(actions::Watch::new())
    }

    fn roleblocker(self) -> Self {
        self
            .with(attributes::Visiting)
            .with(actions::Block::new())
    }

    fn doctor(self) -> Self {
        self
            .with(attributes::Visiting)
            .with(actions::Save::new())
    }

    fn goon(self) -> Self {
        self
            .with(Faction::Mafia)
            .with(attributes::Visiting)
            .with(attributes::Gun)
            .with(actions::Kill::new())
    }

    fn godfather(self) -> Self {
        self
            .with(Faction::Mafia)
            .with(attributes::Visiting)
            .with(attributes::Innocence::Innocent)
            .with(attributes::Undetectable)
            .with(actions::Kill::new())
    }

}

trait ModifierBuilder {
    fn breakthrough(self) -> Self;
    fn macho(self) -> Self;
}

impl<'a> ModifierBuilder for EntityBuilder<'a> {
    fn breakthrough(self) -> Self {
        self
            .with(attributes::Breakthrough)
    }
    fn macho(self) -> Self {
        self
            .with(attributes::Macho)
    }
}
