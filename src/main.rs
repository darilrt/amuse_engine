use amuse_core::EngineState;
use amuse_event::EventSystem;
use amuse_scene::{Component, SceneManager};
use amuse_window::Window;

struct TestComponent;

impl Component for TestComponent {
    fn get_name(&self) -> &str {
        "TestComponent"
    }

    fn init(&mut self, _state: &mut EngineState) {
        println!("TestComponent init");
    }

    fn update(&mut self, _state: &mut EngineState) {
        println!("TestComponent update");
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

fn main() {
    let mut st = EngineManager::default();

    st.add_resource(EventSystem::default());
    st.add_resource(SceneManager::default());

    let scene_manager = st.get_resource_mut::<SceneManager>().unwrap();

    let mut scene = amuse_scene::Scene::new();
    let mut object = amuse_scene::Object::new("test");
    object.add_component(TestComponent);
    scene.add_object(object);

    scene_manager.set_scene(scene);

    st.add_resource(Window::default());
}
