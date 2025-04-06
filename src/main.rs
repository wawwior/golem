use app::App;
use winit::event_loop::EventLoop;

mod app;
mod data;
mod direction;
mod face_buffers;
mod render;

#[tracing::instrument]
fn main() {
    tracing_subscriber::fmt().compact().init();

    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    let mut app = App::default();

    event_loop.run_app(&mut app).unwrap();
}
