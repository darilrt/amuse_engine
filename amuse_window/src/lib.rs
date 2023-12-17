use amuse_core::EngineState;
use winit::{
    event::{Event, WindowEvent},
    window,
};

pub struct Window {
    window: Option<winit::window::Window>,
}

impl Default for Window {
    fn default() -> Self {
        Window { window: None }
    }
}

impl amuse_core::Resource for Window {
    fn init(&mut self, st: &mut EngineState) {
        let es = st.get_resource_mut::<amuse_event::EventSystem>().unwrap();

        let event_loop = winit::event_loop::EventLoop::new().unwrap();

        let window = window::WindowBuilder::new().build(&event_loop).unwrap();
        self.window = Some(window);

        let _ = event_loop.run(move |event, elwt| match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } => {
                if window_id == self.window().id() {
                    match event {
                        WindowEvent::Resized(physical_size) => {
                            es.notify(WindowResizedEvent {
                                width: physical_size.width,
                                height: physical_size.height,
                            });
                        }
                        WindowEvent::CloseRequested => {
                            es.notify(CloseWindowEvent {});
                            elwt.exit();
                        }
                        WindowEvent::RedrawRequested => {
                            es.notify(RedrawEvent {});
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        });
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Window {
    fn window(&self) -> &winit::window::Window {
        self.window.as_ref().unwrap()
    }
}

#[derive(amuse_macro::Event)]
pub struct CloseWindowEvent;

#[derive(amuse_macro::Event)]
pub struct WindowResizedEvent {
    pub width: u32,
    pub height: u32,
}

#[derive(amuse_macro::Event)]
pub struct RedrawEvent;
