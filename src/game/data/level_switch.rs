use game::*;
use ecs::*;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum LevelSwitch {
    NewLevel(TerrainType),
    ExistingLevel(LevelExit),
}

// the id of a level, and the id of an entity within that level
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct LevelExit {
    pub level_id: LevelId,
    pub exit_id: EntityId,
}

#[derive(Clone, Copy, Debug)]
pub struct LevelSwitchAction {
    pub entity_id: EntityId,
    pub exit_id: EntityId,
    pub level_switch: LevelSwitch,
}
