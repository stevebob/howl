use game::{
    rule,
    actions,
    ComponentType,
    RuleResult,
    RuleContext,
};

use game::update::Metadatum::*;

pub fn detect_collision(ctx: RuleContext)
    -> RuleResult
{
    for (entity_id, changes) in &ctx.update.added_components {

        if !changes.has(ComponentType::Position) {
            continue;
        }

        let entity = ctx.entities.get(*entity_id);

        if !entity.has(ComponentType::Collider) {
            continue;
        }

        let level = ctx.entities.get(entity.on_level().unwrap());
        let spacial_hash = level.level_spacial_hash().unwrap();

        let new_position = changes.position().unwrap();

        if let Some(cell) = spacial_hash.get(new_position.to_tuple()) {
            if cell.has(ComponentType::Solid) {
                if entity.is_destroy_on_collision() {
                    let mut remove = actions::remove_entity(entity);
                    remove.set_metadata(ActionTime(1));
                    return rule::instead(remove);
                } else {
                    return rule::fail();
                }
            }
        }
    }

    rule::pass()
}