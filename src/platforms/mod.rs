
pub trait PlatformBase {
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}


pub mod softbuffer;
