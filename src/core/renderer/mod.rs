pub mod base;
use base::draw::Draw;

pub mod software;
use software::SoftwareRenderer;

mod renderer_type;
pub use renderer_type::RendererType;


pub fn create_renderer(engine_type: RendererType) -> impl Draw {
    match engine_type {
        RendererType::Software => SoftwareRenderer{},
        RendererType::OpenGL => SoftwareRenderer{},
    }
}