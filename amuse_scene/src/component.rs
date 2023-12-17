use amuse_core::EngineState;

pub trait Component {
    fn get_name(&self) -> &str;
    fn init(&mut self, state: &mut EngineState);
    fn update(&mut self, state: &mut EngineState);

    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}
