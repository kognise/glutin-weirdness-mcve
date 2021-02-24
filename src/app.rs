use glutin::event::Event;
use glutin::event::WindowEvent;
use glutin::event_loop::ControlFlow;
use glutin::event_loop::EventLoop;

use crate::window::Window;

pub struct App {
    pub window: Window,
}

impl App {
    pub fn new(event_loop: &EventLoop<()>, name: &str, width: u32, height: u32) -> Self {
        let window = Window::new(event_loop, name, width, height);
        App { window }
    }

    pub fn main_loop(self, event_loop: EventLoop<()>) {
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::RedrawRequested(_) => {
                    println!("Inner size: {:?}", self.window.handle);
                }

                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        println!("Close requested");
                        *control_flow = ControlFlow::Exit;
                    }

                    _ => (),
                },

                _ => (),
            }
        })
    }
}
