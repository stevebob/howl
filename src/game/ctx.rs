use std::cell::RefCell;

use game::*;
use game::data::*;
use ecs::*;
use frontends::ansi;
use util::{LeakyReserver, Schedule};
use math::Coord;

pub struct GameCtx<'a> {
    levels: LevelTable,
    renderer: AnsiRenderer<'a>,
    input_source: ansi::AnsiInputSource,
    entity_ids: RefCell<LeakyReserver<EntityId>>,
    turn_id: u64,
    level_id: isize,
    pc_id: Option<EntityId>,
    pc_observer: Shadowcast,
    behaviour_ctx: BehaviourCtx,
    rules: Vec<Box<Rule>>,
    rule_resolution: RuleResolution,
    ecs_action: EcsAction,
    action_schedule: Schedule<ActionArgs>,
}

impl<'a> GameCtx<'a> {
    pub fn new(window: ansi::Window<'a>, input_source: ansi::AnsiInputSource) -> Self {
        GameCtx {
            levels: LevelTable::new(),
            renderer: AnsiRenderer::new(window),
            input_source: input_source,
            entity_ids: RefCell::new(LeakyReserver::new()),
            turn_id: 0,
            level_id: 0,
            pc_id: None,
            pc_observer: Shadowcast::new(),
            behaviour_ctx: BehaviourCtx::new(input_source),
            rules: Vec::new(),
            rule_resolution: RuleResolution::new(),
            ecs_action: EcsAction::new(),
            action_schedule: Schedule::new(),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        self.rules.push(Box::new(rules::OpenDoor));
        self.rules.push(Box::new(rules::Collision));
        self.rules.push(Box::new(rules::CloseDoor));
        self.init_demo();

        self.game_loop()
    }

    fn game_loop(&mut self) -> Result<()> {
        loop {

            self.turn_id += 1;

            let level = self.levels.level_mut(self.level_id);
            if let Some(turn_event) = level.turn_schedule.next() {

                let resolution = TurnEnv {
                    turn_id: self.turn_id,
                    level_id: self.level_id,
                    entity_id: turn_event.event,
                    pc_id: self.pc_id.unwrap(),
                    renderer: &mut self.renderer,
                    ecs: &mut level.ecs,
                    spatial_hash: &mut level.spatial_hash,
                    behaviour_ctx: &self.behaviour_ctx,
                    rules: &self.rules,
                    rule_resolution: &mut self.rule_resolution,
                    ecs_action: &mut self.ecs_action,
                    action_schedule: &mut self.action_schedule,
                    pc_observer: &self.pc_observer,
                }.turn()?;

                match resolution {
                    TurnResolution::Quit => return Ok(()),
                    TurnResolution::Schedule(entity_id, delay) => {
                        level.turn_schedule.insert(entity_id, delay);
                    }
                }
            } else {
                return Err(Error::ScheduleEmpty);
            }
        }
    }

    fn new_id(&self) -> EntityId {
        self.entity_ids.borrow_mut().reserve()
    }

    fn commit(&mut self, action: &mut EcsAction) {
        let level = self.levels.level_mut(self.level_id);
        level.spatial_hash.update(Turn::new(&level.ecs, self.turn_id), action);
        level.ecs.commit(action);
    }

    fn init_demo(&mut self) {
        let strings = demo_level_str();

        let mut g = EcsAction::new();

        let mut y = 0;
        for line in &strings {
            let mut x = 0;
            for ch in line.chars() {
                let coord = Coord::new(x, y);
                match ch {
                    '#' => {
                        prototypes::wall(g.entity_mut(self.new_id()), coord);
                        prototypes::floor(g.entity_mut(self.new_id()), coord);
                    }
                    '&' => {
                        prototypes::tree(g.entity_mut(self.new_id()), coord);
                        prototypes::outside_floor(g.entity_mut(self.new_id()), coord);
                    }
                    '.' => {
                        prototypes::floor(g.entity_mut(self.new_id()), coord);
                    }
                    ',' => {
                        prototypes::outside_floor(g.entity_mut(self.new_id()), coord);
                    }
                    '+' => {
                        prototypes::door(g.entity_mut(self.new_id()), coord, DoorState::Closed);
                    }
                    '@' => {
                        let id = self.new_id();
                        self.pc_id = Some(id);
                        prototypes::pc(g.entity_mut(id), coord);
                        prototypes::outside_floor(g.entity_mut(self.new_id()), coord);
                    }
                    _ => panic!(),
                }
                x += 1;
            }
            y += 1;
        }

        self.commit(&mut g);
        self.levels.level_mut(self.level_id).turn_schedule.insert(self.pc_id.unwrap(), 0);
    }
}

fn demo_level_str() -> Vec<&'static str> {
    vec!["&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&",
         "&,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,&",
         "&,,############################,,,,,,&",
         "&,,#.........#................#,,&,,,&",
         "&,,#.........#................#,,,&,,&",
         "&,,#..........................#,,&,,,&",
         "&&,#.........#................#,,,,,,&",
         "&,&#.........##########+#######,,,,,,&",
         "&,,#.........#,,,,,,,,,,,,,,,,,,,,,,,&",
         "&&,#.........#,,,,,,,,,&,,,,,,,&,&,&,&",
         "&,,#.........#,,,,,&,,,,,,,,&,,,,,,,,&",
         "&,,#.........+,,,,,,&,,,,,,,,,,,,,,,,&",
         "&&,#.........#,,,,,&,,,,,,,,,&,,,,,,,&",
         "&,,#.........#,,,,,,,,,,&,,&,,,&,&,,,&",
         "&,&#.........#,,,,@,,,,&,,,,,,,,,,,,,&",
         "&,,###########,,,,,,,&,,,,,,,&,&,,,,,&",
         "&,,&,,,,,,,,,,,,,,,,,&,,,,&,,,,,,,,,,&",
         "&,&,,,,,,,,,,,,&,,,,,,,,,,,,,,,,,,,,,&",
         "&,,,&,,,,,,,,,,,,,,,,&,,,,,#########,&",
         "&,&,,,&,,,,,&,,&,,,,&,,,,,,#.......#,&",
         "&,,,,,&,,,,,,,,,&,,,,&,,,,,#.......#,&",
         "&,,,,,,,,,&,,,,,,,,,,,,,&,,........#,&",
         "&,&,&,,,,&&,,,&,&,,,,,,,&,,#.......#,&",
         "&,,,,,,,,,,,,,,,,,,,&,,,,,,#.......#,&",
         "&,,,&,,,,,,,&,,,,,,,,,,,,,,#########,&",
          "&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&&"]
}


