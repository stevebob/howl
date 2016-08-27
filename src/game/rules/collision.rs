use game::{
    rule,
    actions,
    EntityTable,
    ComponentType,
    UpdateSummary,
};

use game::rule::RuleResult;

pub fn detect_collision(summary: &UpdateSummary,
                        entities: &EntityTable)
    -> RuleResult
{
    for (entity_id, changes) in &summary.added_components {

        if !changes.has(ComponentType::Position) {
            continue;
        }

        let entity = entities.get(*entity_id);

        if !entity.has(ComponentType::Collider) {
            continue;
        }

        let level = entities.get(entity.on_level().unwrap());
        let spacial_hash = level.level_spacial_hash().unwrap();

        let new_position = changes.position().unwrap();

        if let Some(cell) = spacial_hash.get(new_position.to_tuple()) {
            if cell.has(ComponentType::Solid) {
                if entity.is_destroy_on_collision() {
                    return rule::instead(actions::remove_entity(entity));
                } else {
                    return rule::fail();
                }
            }
        }
    }

    rule::pass()
}
