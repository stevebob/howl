use rand::Rng;
use math::Vector2;
use coord::Coord;

use ecs::*;
use game::*;
use game::data::*;

pub const ENV_TURN_OFFSET: u64 = 0;
pub const NPC_TURN_OFFSET: u64 = 1;
pub const PC_TURN_OFFSET: u64 = 2;

pub fn wall<E: EntityPopulate>(mut entity: E, position: Coord) -> E {
    entity.insert_position(position);
    entity.insert_opacity(1.0);
    entity.insert_solid();

    entity.insert_tile(TileType::Wall);

    entity.insert_tile_depth(1);

    entity
}

pub fn tree(action: &mut EcsAction, ids: &EntityIdReserver, position: Coord) -> EntityId {

    let shadow_id = {
        let mut entity = action.entity_mut(ids.new_id());

        entity.insert_tile(TileType::DeadTree);

        entity.insert_transformation_state(TransformationState::Other);
        entity.insert_opacity(0.0);

        entity.id()
    };

    let mut entity = action.entity_mut(ids.new_id());
    entity.insert_position(position);
    entity.insert_opacity(0.6);
    entity.insert_solid();

    entity.insert_tile(TileType::Tree);
    entity.insert_you_see(YouSeeMessageType::Tree);

    entity.insert_tile_depth(1);
    entity.insert_shadow_entity(shadow_id);
    entity.insert_transformation_state(TransformationState::Real);
    entity.insert_transformation_type(TransformationType::Tree);

    entity.id()
}

pub fn floor<E: EntityPopulate>(mut entity: E, position: Coord) -> E {
    entity.insert_position(position);

    entity.insert_tile(TileType::Floor);

    entity.insert_tile_depth(0);
    entity.insert_floor();

    entity
}

pub fn outside_floor<E: EntityPopulate>(mut entity: E, position: Coord) -> E {
    entity.insert_position(position);

    entity.insert_tile(TileType::Ground);

    entity.insert_tile_depth(0);
    entity.insert_floor();

    entity
}

pub fn pc<E: EntityPopulate>(mut entity: E, position: Coord) -> E {
    entity.insert_position(position);

    entity.insert_tile(TileType::Player);

    entity.insert_tile_depth(2);
    entity.insert_collider();
    entity.insert_behaviour_state(BehaviourState::new());
    entity.insert_behaviour_type(BehaviourType::PlayerInput);
    entity.insert_turn_offset(PC_TURN_OFFSET);
    entity.insert_drawable_knowledge(DrawableKnowledge::new());
    entity.insert_vision_distance(16);
    entity.insert_door_opener();
    entity.insert_pc();
    entity.insert_turn_time(TURN_DURATION_BASE);
    entity.insert_should_render();
    entity.insert_message_log(MessageLog::new());
    entity.insert_you_see(YouSeeMessageType::Player);
    entity.insert_description(DescriptionMessageType::Player);
    entity.insert_projectile_collider();
    entity.insert_hit_points(HitPoints::new(10));
    entity.insert_bump_attackable();

    entity
}

pub fn terror_pillar(action: &mut EcsAction, ids: &EntityIdReserver, position: Coord) -> EntityId {

    let shadow_id = {
        let mut entity = action.entity_mut(ids.new_id());

        entity.insert_tile(TileType::TerrorFly);

        entity.insert_turn_time(TURN_DURATION_BASE / 2);
        entity.insert_transformation_state(TransformationState::Other);

        entity.id()
    };

    let mut entity = action.entity_mut(ids.new_id());
    entity.insert_position(position);

    entity.insert_tile(TileType::TerrorPillar);

    entity.insert_tile_depth(2);
    entity.insert_collider();
    entity.insert_behaviour_state(BehaviourState::new());
    entity.insert_behaviour_type(BehaviourType::SimpleNpc);
    entity.insert_turn_offset(NPC_TURN_OFFSET);
    entity.insert_vision_distance(8);
    entity.insert_simple_npc_knowledge(SimpleNpcKnowledge::new());
    entity.insert_path_traverse(PathTraverse::new());
    entity.insert_turn_time(TURN_DURATION_BASE * 2);
    entity.insert_shadow_entity(shadow_id);
    entity.insert_transformation_type(TransformationType::TerrorPillarTerrorFly);
    entity.insert_transformation_state(TransformationState::Real);
    entity.insert_enemy();
    entity.insert_projectile_collider();
    entity.insert_hit_points(HitPoints::new(2));
    entity.insert_bump_attacker(1);

    entity.id()
}


pub fn door<E: EntityPopulate>(mut entity: E, position: Coord, state: DoorState) -> E {
    entity.insert_position(position);

    if state.is_open() {
        entity.insert_tile(TileType::OpenDoor);
        entity.insert_opacity(0.0);
    } else {
        entity.insert_tile(TileType::ClosedDoor);
        entity.insert_solid();
        entity.insert_opacity(1.0);
    }
    entity.insert_tile_depth(1);
    entity.insert_door_state(state);

    entity
}

pub fn bullet<E: EntityPopulate>(mut entity: E, position: Coord, velocity: RealtimeVelocity) -> E {

    entity.insert_position(position);
    entity.insert_realtime_velocity(velocity);
    entity.insert_destroy_on_collision();
    entity.insert_projectile();
    entity.insert_projectile_damage(1);

    entity.insert_tile(TileType::Bullet);

    entity.insert_tile_depth(1);

    entity
}

pub fn tear<E: EntityPopulate, R: Rng>(mut entity: E, width: usize, height: usize, r: &mut R) -> E {

    const PERLIN_X_ZOOM: f64 = 0.05;
    const PERLIN_Y_ZOOM: f64 = 0.05;
    const PERLIN_SIZE: f64 = 0.1;
    const SCROLL_RATE: Vector2<f64> = Vector2 { x: 0.05, y: 0.02 };
    const MUTATE_RATE: f64 = 0.01;

    entity.insert_tear_state(TearState::new(width, height, PERLIN_X_ZOOM, PERLIN_Y_ZOOM,
                                              PERLIN_SIZE, SCROLL_RATE, MUTATE_RATE, r));
    entity.insert_behaviour_state(BehaviourState::new());
    entity.insert_behaviour_type(BehaviourType::Tear);
    entity.insert_turn_offset(ENV_TURN_OFFSET);
    entity.insert_turn_time(TURN_DURATION_BASE);

    entity
}

pub fn book<E: EntityPopulate>(mut entity: E, position: Coord, level_switch: LevelSwitch) -> E {

    entity.insert_position(position);
    entity.insert_tile(TileType::Book);
    entity.insert_tile_depth(1);
    entity.insert_level_switch(level_switch);
    entity.insert_level_switch_auto();

    entity
}

pub fn down_stairs<E: EntityPopulate>(mut entity: E, position: Coord, level_switch: LevelSwitch, group: Option<usize>) -> E {

    entity.insert_position(position);
    entity.insert_tile(TileType::DownStairs);
    entity.insert_tile_depth(1);
    entity.insert_level_switch(level_switch);

    group.map(|group| entity.insert_level_switch_group(group));

    entity
}

pub fn up_stairs<E: EntityPopulate>(mut entity: E, position: Coord, level_switch: LevelSwitch, group: Option<usize>) -> E {

    entity.insert_position(position);
    entity.insert_tile(TileType::UpStairs);
    entity.insert_tile_depth(1);
    entity.insert_level_switch(level_switch);

    group.map(|group| entity.insert_level_switch_group(group));

    entity
}
