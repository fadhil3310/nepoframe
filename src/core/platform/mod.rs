// pub mod winit;
pub mod softbuffer;

pub mod platform_list;
use platform_list::PlatformList;

pub fn create_platform(platform: PlatformList) {
    match platform {
        PlatformList::Softbuffer => {
            
        }
    }
}