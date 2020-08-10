pub mod dynamic_nodes;
pub mod hexgrid;
use crate::game_state::GameState;
use dynamic_nodes::{create_nodes, update_nodes};
use gdnative::prelude::*;
use lazy_static::lazy_static;
use legion::prelude::*;
use std::sync::Mutex;

lazy_static! {
    static ref GAMESTATE: Mutex<GameState> = Mutex::new(GameState::new());
}

pub fn with_game_state<F>(mut f: F)
where
    F: FnMut(&mut GameState),
{
    let _result = GAMESTATE.try_lock().map(|mut state| f(&mut state));
}

pub struct Delta(pub f32);

pub struct HexfieldSize(pub i32);

pub struct Selected(pub bool);

unsafe impl Send for Selected {}
unsafe impl Sync for Selected {}

impl Clone for Selected {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl PartialEq for Selected {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

pub struct UpdateNotes {
    resources: Resources,
    schedule: Schedule,
    pub hexfield_size: i32,
}

impl UpdateNotes {
    pub fn new(hexfield_size: i32) -> Self {
        let mut resources = Resources::default();
        resources.insert(HexfieldSize(hexfield_size));

        let schedule = Schedule::builder().add_thread_local(update_nodes()).build();
        Self {
            resources,
            schedule,
            hexfield_size,
        }
    }

    pub fn execute(&mut self, root: &Node2D, _delta: f64) {
        self.resources
            .get_mut::<HexfieldSize>()
            .map(|mut d| d.0 = self.hexfield_size);

        with_game_state(|state| {
            create_nodes(&mut state.world, root);
        });

        with_game_state(|state| {
            self.schedule.execute(&mut state.world, &mut self.resources);
        })
    }
}
