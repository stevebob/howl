use game::entity::{EntityTable, ComponentType};
use game::entity::Component::*;
use game::components::door::DoorState;
use game::update::UpdateSummary;

use game::rule::RuleResult;
use game::rule;
use game::game_entity::GameEntity;
use game::actions;

pub fn detect_open(summary: &UpdateSummary,
                    entities: &EntityTable)
    -> RuleResult
{
    if !summary.changed_components.contains(&ComponentType::Position) {
        return rule::pass();
    }

    for (entity_id, components) in &summary.changed_entities {

        if !components.contains_key(&ComponentType::Position) {
            continue;
        }

        let entity = entities.get(*entity_id);
        let level = entities.get(entity.on_level().unwrap());

        if !entity.has(ComponentType::Collider) {
            continue;
        }

        let spacial_hash = level.level_spacial_hash().unwrap();

        let current_position = entity.position().unwrap();

        if let Some(cell) = spacial_hash.get(current_position.to_tuple()) {
            if cell.has(ComponentType::Door) && cell.has(ComponentType::Solid) {
                for entity_id in &cell.entities {
                    if let Some(&Door(DoorState::Closed)) = entities.get(*entity_id).get(ComponentType::Door) {
                        return RuleResult::Instead(vec![
                            actions::open_door(*entity_id),
                        ]);
                    }
                }
            }
        }
    }

    rule::pass()
}
