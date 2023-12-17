use amuse_core::EngineState;

use crate::object::Object;

#[derive(Default)]
pub struct Scene {
    pub objects: Vec<Object>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn get_object(&mut self, name: &str) -> Option<&mut Object> {
        for object in &mut self.objects {
            if object.name == name {
                return Some(object);
            }
        }

        None
    }

    pub fn init(&mut self, state: &mut EngineState) {
        for object in &mut self.objects {
            object.init(state);
        }
    }

    pub fn update(&mut self, state: &mut EngineState) {
        for object in &mut self.objects {
            object.update(state);
        }
    }
}
