use crate::resource::Resource;

pub struct EngineState {
    resources: Vec<Box<dyn Resource>>,
}

impl EngineState {
    pub fn new() -> Self {
        EngineState {
            resources: Vec::new(),
        }
    }

    pub fn add_resource<T: Resource + 'static + Default>(&mut self, t: T) {
        let mut resource = Box::new(t);
        resource.init(self);
        self.resources.push(resource);
    }

    pub fn get_resource<T: Resource + 'static>(&self) -> Option<&T> {
        for resource in &self.resources {
            if let Some(resource) = resource.as_any().downcast_ref::<T>() {
                return Some(resource);
            }
        }

        None
    }

    pub fn get_resource_mut<T: Resource + 'static>(&mut self) -> Option<&mut T> {
        for resource in &mut self.resources {
            if let Some(resource) = resource.as_any_mut().downcast_mut::<T>() {
                return Some(resource);
            }
        }

        None
    }
}
