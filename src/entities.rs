use specs::{World,WorldExt,Entity,EntityBuilder,Builder};
use super::components::{Name,Faction,Target,Position,Role,Modifier,actions,attributes};

// certain roles will overwrite the faction
// e.g. if trying to make a Town Goon, a Mafia Goon will instead be returned
pub fn create_player(world: &mut World, name: String, faction: Faction, role: Role, modifiers: Vec<Modifier>) -> Entity {
    let player = world.create_entity()
        .base_player(name)
        .faction(faction);
    let mut player = give_role(player, role);
    for modifier in modifiers {
        player = give_modifier(player, modifier);
    }
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

pub fn give_modifier<'a>(player: EntityBuilder<'a>, modifier: Modifier) -> EntityBuilder<'a> {
    let player = player.with(modifier.clone());
    match modifier {
        Modifier::Breakthrough => player.breakthrough(),
        Modifier::Macho        => player.macho(),
    }
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
            .with(actions::Cop)
    }

    fn detective(self) -> Self {
        self
            .with(attributes::Visiting)
            .with(attributes::Gun)
            .with(actions::Detective)
    }

    fn tracker(self) -> Self {
        self
            .with(attributes::Visiting)
            .with(actions::Track)
    }

    fn watcher(self) -> Self {
        self
            .with(attributes::Visiting)
            .with(actions::Watch)
    }

    fn roleblocker(self) -> Self {
        self
            .with(attributes::Visiting)
            .with(actions::Block)
    }

    fn doctor(self) -> Self {
        self
            .with(attributes::Visiting)
            .with(actions::Save)
    }

    fn goon(self) -> Self {
        self
            .with(Faction::Mafia)
            .with(attributes::Visiting)
            .with(attributes::Gun)
            .with(actions::Kill)
    }

    fn godfather(self) -> Self {
        self
            .with(Faction::Mafia)
            .with(attributes::Visiting)
            .with(attributes::Innocence::Innocent)
            .with(attributes::Undetectable)
            .with(actions::Kill)
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
