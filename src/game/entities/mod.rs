use game;
use game::entity::{Entity, EntityId};
use game::entity::Component::*;
use game::components::{
    Level,
    DoorState,
};
use game::knowledge;

use geometry::Vector2;
use renderer::{
    tile,
    ComplexTile,
    SimpleTile,
};
use colour::ansi;

use std::cell::RefCell;

pub fn make_wall(x: isize, y: isize, level: EntityId) -> Entity {
    entity![
        Position(Vector2::new(x, y)),
        Solid,
        Tile(ComplexTile::Wall {
            front: SimpleTile::Full { ch: '▄', fg: ansi::MAGENTA, bg: ansi::GREY },
            back: SimpleTile::Foreground('█', ansi::GREY),
        }),
        TileDepth(1),
        OnLevel(level),
        Opacity(1.0),
    ]
}

pub fn make_door(x: isize, y: isize, level: EntityId, state: DoorState) -> Entity {
    let mut entity = entity![
        Position(Vector2::new(x, y)),
        TileDepth(1),
        OnLevel(level),
    ];

    if state == DoorState::Open {
        entity.add(Tile(tile::foreground('-', ansi::WHITE)));
        entity.add(Door(DoorState::Open));
        entity.add(Opacity(0.0));
    } else {
        entity.add(Solid);
        entity.add(Tile(tile::full('+', ansi::WHITE, ansi::DARK_GREY)));
        entity.add(Door(DoorState::Closed));
        entity.add(Opacity(1.0));
    }

    entity
}

pub fn make_tree(x: isize, y: isize, level: EntityId) -> Entity {
    entity![
        Position(Vector2::new(x, y)),
        Solid,
        Tile(tile::foreground('&', ansi::GREEN)),
        TileDepth(1),
        OnLevel(level),
        Opacity(0.4),
    ]
}

pub fn make_floor(x: isize, y: isize, level: EntityId) -> Entity {
    entity![
        Position(Vector2::new(x, y)),
        Tile(tile::full('.', ansi::WHITE, ansi::DARK_GREY)),
        TileDepth(0),
        OnLevel(level),
    ]
}

pub fn make_pc(x: isize, y: isize, level: EntityId) -> Entity {
    entity![
        Position(Vector2::new(x, y)),
        Tile(tile::foreground('@', ansi::WHITE)),
        TileDepth(2),
        PlayerActor,
        OnLevel(level),
        Collider,
        DoorOpener,
        VisionDistance(20),
        DrawableKnowledge(RefCell::new(knowledge::DrawableKnowledge::new())),
    ]
}

pub fn make_bullet(x: isize, y: isize, level: EntityId) -> Entity {
    entity![
        Position(Vector2::new(x, y)),
        Tile(tile::foreground('*', ansi::RED)),
        TileDepth(2),
        OnLevel(level),
        Collider,
        Bullet,
        DestroyOnCollision,
    ]
}

pub fn make_level(width: usize, height: usize) -> Entity {
    entity![
        LevelData(Level::new(width, height))
    ]
}