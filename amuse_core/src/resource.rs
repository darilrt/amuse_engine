use crate::state::EngineState;

pub trait Resource {
    fn init(&mut self, state: &mut EngineState);
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}
