use std::sync::{Arc, Mutex, MutexGuard};

use crate::resource::Resource;

pub struct EngineManager {
    manager: Arc<Mutex<EngineState>>,
}
pub struct EngineState {
    resources: Vec<Box<dyn Resource>>,
}

impl EngineManager {
    pub fn manager(&mut self) -> MutexGuard<'_, EngineState> {
        self.manager.lock().unwrap()
    }

    pub fn add_resource<T: Resource + 'static>(&mut self, t: T) {
        self.manager().add_resource(t, self);
    }
}

impl EngineState {
    pub fn add_resource<T: Resource + 'static>(&mut self, t: T, manager: &mut EngineManager) {
        let mut resource = Box::new(t);
        resource.init(manager);
        self.resources.push(resource);
    }

    pub fn get_resource<T: Resource + 'static>(&mut self) -> Option<&T> {
        self.resources
            .iter_mut()
            .find_map(|r| r.as_any().downcast_ref())
    }

    pub fn get_resource_mut<T: Resource + 'static>(&mut self) -> Option<&mut T> {
        None
    }
}
