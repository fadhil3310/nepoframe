mod core;
use core::platform::softbuffer;
// use core::renderer;


fn main() -> Result<(), ::winit::error::EventLoopError> {
    // let engine = renderer::create_renderer(renderer::RendererType::Software);

    let result: Result<(), winit::error::EventLoopError> = softbuffer::run();
    result
}
