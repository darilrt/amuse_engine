use amuse_core::EngineState;

use crate::component::Component;

pub struct Object {
    pub name: String,
    pub components: Vec<Box<dyn Component>>,
    pub children: Vec<Object>,
}

impl Object {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            components: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn add_component<T: Component + 'static>(&mut self, component: T) {
        self.components.push(Box::new(component));
    }

    pub fn get_component<T: Component + 'static>(&mut self) -> Option<&mut T> {
        for component in &mut self.components {
            if let Some(component) = component.as_any_mut().downcast_mut::<T>() {
                return Some(component);
            }
        }

        None
    }

    pub fn add_child(&mut self, child: Object) {
        self.children.push(child);
    }

    pub fn get_child(&mut self, name: &str) -> Option<&mut Object> {
        for child in &mut self.children {
            if child.name == name {
                return Some(child);
            }
        }

        None
    }

    pub fn init(&mut self, state: &mut EngineState) {
        for component in &mut self.components {
            component.init(state);
        }

        for child in &mut self.children {
            child.init(state);
        }
    }

    pub fn update(&mut self, state: &mut EngineState) {
        for component in &mut self.components {
            component.update(state);
        }

        for child in &mut self.children {
            child.update(state);
        }
    }
}
