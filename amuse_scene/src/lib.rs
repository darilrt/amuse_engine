use std::ops::Deref;

use amuse_core::{EngineState, Resource};
use amuse_event::{Event, EventSystem};
use amuse_events::*;
pub use component::Component;
pub use object::Object;
pub use scene::Scene;

mod component;
mod object;
mod scene;

#[derive(Default)]
pub struct SceneManager {
    scene: Option<Scene>,
}

impl Resource for SceneManager {
    fn init(&mut self, state: &mut EngineState) {
        let event_system = state.get_resource_mut::<EventSystem>().unwrap();

        event_system.subscribe::<LoopInit>(|e: &dyn Event| {});

        event_system.subscribe::<LoopUpdate>(|_e| {});
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl SceneManager {
    pub fn new() -> Self {
        Self { scene: None }
    }

    pub fn set_scene(&mut self, scene: Scene) {
        self.scene = Some(scene);
    }

    pub fn get_scene(&mut self) -> &mut Scene {
        self.scene.as_mut().expect("No scene set")
    }

    pub fn init(&mut self, state: &mut EngineState) {
        self.scene.as_mut().expect("No scene set").init(state);
    }

    pub fn update(&mut self, state: &mut EngineState) {
        self.scene.as_mut().expect("No scene set").update(state);
    }
}
