use std::rc::Rc;
use winit::application::ApplicationHandler;
use winit::event::{WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

use crate::platforms::PlatformBase;

pub struct SoftbufferPlatform {
    event_loop: Option<EventLoop<()>>,
    window: Option<Rc<Window>>,
    surface: Option<softbuffer::Surface<Rc<Window>, Rc<Window>>>,
}

impl PlatformBase for SoftbufferPlatform {
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(event_loop) = self.event_loop.take() {
            event_loop.run_app(self)?;
        } else {
            eprintln!("Platform has been run");
        }
        Ok(())
    }
}

impl SoftbufferPlatform {
    pub fn new() -> SoftbufferPlatform {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Wait);

        SoftbufferPlatform {
            event_loop: Some(event_loop),
            window: None,
            surface: None,
        }
    }
}

impl ApplicationHandler for SoftbufferPlatform {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = {
            let window = event_loop
                .create_window(Window::default_attributes())
                .unwrap();
            Rc::new(window)
        };

        let context = softbuffer::Context::new(window.clone()).unwrap();
        self.surface = Some(softbuffer::Surface::new(&context, window.clone()).unwrap());
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let (width, height) = {
                    let size = self.window.as_ref().unwrap().inner_size();
                    (size.width, size.height)
                };

                let surface = self.surface.as_mut().unwrap();

                surface
                    .resize(
                        std::num::NonZeroU32::new(width).unwrap(),
                        std::num::NonZeroU32::new(height).unwrap(),
                    )
                    .unwrap();

                let mut buffer = surface.buffer_mut().unwrap();
                // for index in 0..(width * height) {
                //     let y = index / width;
                //     let x = index % width;
                //     let red = x % 255;
                //     let green = y % 255;
                //     let blue = (x * y) % 255;

                //     buffer[index as usize] = blue | (green << 8) | (red << 16);
                // }
                for index in 0..(width * height) {
                    // let y = index / width;
                    // let x = index % width;
                    let red = 255;
                    let green = 255;
                    let blue = 255;

                    buffer[index as usize] = blue | (green << 8) | (red << 16);
                }

                buffer.present().unwrap();

                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // Draw.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                // self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}
