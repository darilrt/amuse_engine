use std::{collections::HashMap, sync::Mutex};

use amuse_core::{EngineState, Resource};

pub trait Event {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

#[derive(Default)]
pub struct EventSystem {
    pub events: HashMap<std::any::TypeId, Vec<fn(&dyn Event)>>,
    pub state: Option<Mutex<EngineState>>,
}

impl Resource for EventSystem {
    fn init(&mut self, st: &mut EngineState) {
        self.state.replace(st.lock().unwrap().clone());
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl EventSystem {
    pub fn subscribe<T: Event + 'static>(&mut self, handler: fn(&dyn Event)) {
        let t: std::any::TypeId = std::any::TypeId::of::<T>();
        let handlers = self.events.entry(t).or_insert(Vec::new());
        handlers.push(handler);
    }

    pub fn notify<T: Event + 'static>(&mut self, e: T) {
        let t: std::any::TypeId = std::any::TypeId::of::<T>();
        if let Some(handlers) = self.events.get(&t) {
            for handler in handlers {
                handler(&e);
            }
        }
    }
}
