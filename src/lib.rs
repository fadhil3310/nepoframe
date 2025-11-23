use crate::platforms::PlatformBase;



pub struct NepoframeInstance<P: PlatformBase> {
    pub platform: P
}

impl<P: PlatformBase> NepoframeInstance<P> {
    pub fn new(platform: P) -> NepoframeInstance<P> {
        NepoframeInstance { platform }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.platform.run()
    }
}


pub mod renderer;
pub mod platforms;
pub mod views;