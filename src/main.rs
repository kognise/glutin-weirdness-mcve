use app::App;
use glutin::event_loop::EventLoop;

mod app;
mod window;

fn main() {
    let event_loop = EventLoop::new();
    let app = App::new(&event_loop, "test window", 1024, 512);
    app.main_loop(event_loop);
}
