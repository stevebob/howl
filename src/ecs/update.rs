use ecs::entity::{EntityId, Entity, ComponentType, Component};

use std::fmt;

#[derive(Debug)]
pub enum UpdateStage {
    Propose,
    Commit,
}

pub enum Update {
    SetEntityComponent {
        entity_id: EntityId,
        component_type: ComponentType,
        component_value: Component,
    },
    AddEntity(Entity),
    WithEntity(EntityId, Box<Fn(&mut Entity)>),

    Null,

    ThenWithEntity(Box<Update>, Box<Fn(EntityId) -> Update>),
    Then(Box<Update>, Box<Update>),
}

impl fmt::Debug for Update {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Update {{}}")
    }
}

pub fn then_with_entity<F: 'static + Fn(EntityId) -> Update>(update: Update, f: F) -> Update {
    Update::ThenWithEntity(
        Box::new(update),
        Box::new(f)
    )
}

pub fn then(first: Update, second: Update) -> Update {
    Update::Then(
        Box::new(first),
        Box::new(second)
    )
}

pub fn with_entity<F: 'static + Fn(&mut Entity)>(id: EntityId, f: F) -> Update {
    Update::WithEntity(id, Box::new(f))
}
