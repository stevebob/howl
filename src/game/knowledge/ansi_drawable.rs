use game::{LevelKnowledge, SpatialHashCell, Turn};
use grid::DynamicGrid;
use util::BestMap;
use math::Coord;
use frontends::ansi::ComplexTile;

pub struct AnsiDrawableKnowledgeCell {
    last_updated: u64,
    foreground: BestMap<isize, ComplexTile>,
    background: BestMap<isize, ComplexTile>,
}

impl AnsiDrawableKnowledgeCell {
    fn new() -> Self {
        AnsiDrawableKnowledgeCell {
            last_updated: 0,
            foreground: BestMap::new(),
            background: BestMap::new(),
        }
    }
}

impl Default for AnsiDrawableKnowledgeCell {
    fn default() -> Self {
        Self::new()
    }
}

pub struct AnsiDrawableKnowledgeLevel {
    grid: DynamicGrid<AnsiDrawableKnowledgeCell>,
}

impl AnsiDrawableKnowledgeLevel {
    pub fn new() -> Self {
        AnsiDrawableKnowledgeLevel {
            grid: DynamicGrid::new(),
        }
    }
}

impl LevelKnowledge for AnsiDrawableKnowledgeLevel {
    fn update_cell(&mut self, coord: Coord, world_cell: &SpatialHashCell, _accuracy: f64, turn: Turn) {
        let knowledge_cell = self.grid.get_mut_with_default(coord);
        if knowledge_cell.last_updated < world_cell.last_updated() {
            for entity in turn.ecs.entity_iter(world_cell.entity_id_iter()) {
                entity.tile_depth().map(|depth| {
                    entity.ansi_tile().map(|tile| {
                        knowledge_cell.foreground.insert(depth, tile);
                        if tile.opaque_bg() {
                            knowledge_cell.background.insert(depth, tile);
                        }
                    });
                });
            }
        }
        knowledge_cell.last_updated = turn.id;
    }
}
