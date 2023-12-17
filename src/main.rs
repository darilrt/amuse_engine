use amuse_core::EngineState;
use amuse_event::{Event, EventSystem};
use amuse_window::{Window, WindowResizedEvent};

fn main() {
    let mut st = EngineState::new();

    st.add_resource(EventSystem::default());

    let es = st.get_resource_mut::<EventSystem>().unwrap();

    es.subscribe::<WindowResizedEvent>(|e: &dyn Event| {
        let e = e.as_any().downcast_ref::<WindowResizedEvent>().unwrap();
        println!("Window resized to {}x{}", e.width, e.height);
    });

    st.add_resource(Window::default());
}
