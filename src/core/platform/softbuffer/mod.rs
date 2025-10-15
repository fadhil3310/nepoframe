use std::borrow::{Borrow, BorrowMut};
use std::num::NonZeroU32;
use std::rc::Rc;

use softbuffer::Surface;
use winit::application::ApplicationHandler;
use winit::event::{self, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};


#[derive(Default)]
struct App {
  window: Option<Rc<Window>>,
  surface: Option<Surface<Rc<Window>, Rc<Window>>>,
}

impl ApplicationHandler for App {
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    let window = {
      let window = event_loop
        .create_window(Window::default_attributes());
      Rc::new(window.unwrap())
    };

    let context = softbuffer::Context::new(window.clone()).unwrap();
    self.surface = Some(softbuffer::Surface::new(&context, window.clone()).unwrap());

    self.window = Some(window);
  }

  fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
    match event {
      WindowEvent::CloseRequested => {
        println!("The close button was pressed; stopping");
        event_loop.exit();
      }
      WindowEvent::RedrawRequested => {
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

        let window_ref = self.window.as_mut().unwrap();
        let surface_ref = self.surface.as_mut().unwrap();

        let (width, height) = {
          let size = window_ref.inner_size();
          (size.width, size.height)
        };
        surface_ref
          .resize(
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
          )
          .unwrap();

        let mut buffer = surface_ref.buffer_mut().unwrap();
        for index in 0..(width * height) {
          // let y = index / width;
          // let x = index % width;
          let red = 50;
          let green = 25;
          let blue = 70;

          buffer[index as usize] = blue | (green << 8) | (red << 16);
        }

        buffer.present().unwrap();

        // self.window.as_ref().unwrap().request_redraw();
      }
      _ => (),
    }
  }
}

pub fn run() -> Result<(), winit::error::EventLoopError> {
  let event_loop = EventLoop::new().unwrap();

  // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
  // dispatched any events. This is ideal for games and similar applications.

  // ControlFlow::Wait pauses the event loop if no events are available to process.
  // This is ideal for non-game applications that only update in response to user
  // input, and uses significantly less power/CPU time than ControlFlow::Poll.
  event_loop.set_control_flow(ControlFlow::Wait);

  let mut app = App::default();
  event_loop.run_app(&mut app)
  // (app, event_loop)
}