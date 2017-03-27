use std::collections::BTreeMap;

use engine_defs::LevelId;
use content_types::*;

use game::*;
use spatial_hash::*;
use math::Coord;

/// Trait implemented by representations of knowledge about a level
pub trait LevelKnowledge {
    /// Updates a cell of the knowledge representation, returnig true iff the
    /// knowledge of the cell changed as a result of the update.
    fn update_cell(&mut self, coord: Coord, world_cell: &SpatialHashCell, accuracy: f64, action_env: ActionEnv) -> bool;
}

pub trait KnowledgeCell {
    fn update(&mut self, world_cell: &SpatialHashCell, accuracy: f64, action_env: ActionEnv) -> bool;
}

#[derive(Serialize, Deserialize)]
pub struct GameKnowledge<K> {
    levels: BTreeMap<LevelId, K>,
}

impl<K> GameKnowledge<K> {
    pub fn new() -> Self {
        GameKnowledge {
            levels: BTreeMap::new(),
        }
    }

    pub fn level(&self, level_id: LevelId) -> &K {
        self.levels.get(&level_id).expect("No such level")
    }

    pub fn level_mut(&mut self, level_id: LevelId) -> &mut K {
        self.levels.get_mut(&level_id).expect("No such level")
    }
}

impl<K: Default> Default for GameKnowledge<K> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: LevelKnowledge + TwoDimensionalCons> GameKnowledge<K> {
    pub fn level_mut_or_insert_size(&mut self, level_id: LevelId,
                                    width: usize, height: usize) -> &mut K {
        self.levels.entry(level_id).or_insert_with(|| K::new(width, height))
    }
}
