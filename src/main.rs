mod game;
mod mgfw;

const TITLE: &str = "mgfw test";
const XRES: i32 = 320;
const YRES: i32 = 200;

fn main() {
    let el = glutin::event_loop::EventLoop::new();
    let mut core = mgfw::Core::new(TITLE, XRES, YRES, &el);

    el.run(move |event, _, control_flow| {
        if !core.check_events(&event) {
            *control_flow = glutin::event_loop::ControlFlow::Exit;
        }
    });
}
