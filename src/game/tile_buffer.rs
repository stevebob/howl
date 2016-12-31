use game::*;
use grid::{Grid, DefaultGrid, StaticGrid, CoordIterGrid, IterGrid};
use math::Coord;
use direction::Direction;

pub type TileBufferCoordIter = <StaticGrid<CellDrawInfo> as CoordIterGrid>::CoordIter;
pub type TileBufferIter<'a> = <StaticGrid<CellDrawInfo> as IterGrid<'a>>::Iter;

pub struct TileBuffer {
    grid: StaticGrid<CellDrawInfo>,
}

impl TileBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        TileBuffer {
            grid: StaticGrid::new_default(width, height),
        }
    }

    fn is_front(coord: Coord, knowledge: &DrawableKnowledgeLevel) -> bool {
        let south_coord = coord + Direction::South.vector();
        let cell = knowledge.get_with_default(south_coord);
        if let Some(tile) = cell.foreground() {
            if tile.has_front_variant() {
                return false;
            }
        }

        if let Some(tile) = cell.background() {
            if tile.has_front_variant() {
                return false;
            }
        }

        true
    }

    pub fn coord_iter(&self) -> TileBufferCoordIter {
        self.grid.coord_iter()
    }

    pub fn iter(&self) -> TileBufferIter {
        self.grid.iter()
    }

    pub fn update(&mut self, knowledge: &DrawableKnowledgeLevel,
                  turn_id: u64, scroll_position: Option<Coord>) {

        let offset = if let Some(position) = scroll_position {
            position - Coord::new(self.grid.width() as isize / 2, self.grid.height() as isize / 2)
        } else {
            Coord::new(0, 0)
        };

        for (coord, mut cell) in izip!(self.grid.coord_iter(), self.grid.iter_mut()) {
            let world_coord = coord + offset;
            let knowledge_cell = knowledge.get_with_default(world_coord);
            cell.background = knowledge_cell.background();
            cell.foreground = knowledge_cell.foreground();
            cell.moon = knowledge_cell.moon();
            cell.visible = knowledge_cell.last_updated() == turn_id;
            cell.front = Self::is_front(world_coord, knowledge);
        }
    }
}
