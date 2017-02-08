use game::*;
use ecs::*;

pub fn moon_transform(env: RuleEnv, action: &EcsAction, reactions: &mut Vec<Reaction>) -> RuleResult {

    for entity_id in action.moon().insertion_iter() {
        if let Some(position) = env.ecs.position(entity_id) {
            let cell = env.spatial_hash.get(position);
            for transformer_id in cell.transform_on_moon_change_iter() {
                let transformer = env.ecs.entity(transformer_id);
                let transformation_state = transformer.transformation_state()
                    .expect("Entity missing transformation_state");

                if transformation_state == TransformationState::Real {
                    let transformation_type = transformer.transformation_type()
                        .expect("Entity missing transformation_type");
                    let action_args = transformation_type.to_action_args(transformer_id);
                    reactions.push(Reaction::new(action_args, 0));
                }
            }
        }
    }

    for entity_id in action.moon().removal_iter() {
        if let Some(position) = env.ecs.position(entity_id) {
            let cell = env.spatial_hash.get(position);
            for transformer_id in cell.transform_on_moon_change_iter() {
                let transformer = env.ecs.entity(transformer_id);
                let transformation_state = transformer.transformation_state()
                    .expect("Entity missing transformation_state");

                if transformation_state == TransformationState::Other {
                    let transformation_type = transformer.transformation_type()
                        .expect("Entity missing transformation_type");
                    let action_args = transformation_type.to_action_args(transformer_id);
                    reactions.push(Reaction::new(action_args, 0));
                }
            }
        }
    }

    RULE_ACCEPT
}
