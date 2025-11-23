use nepoframe::NepoframeInstance;
use nepoframe::platforms::{softbuffer::SoftbufferPlatform};

fn main() {
    println!("abc");
   
    let platform = SoftbufferPlatform::new();
    let mut instance = NepoframeInstance::new(platform);
    
    instance.run();
}
