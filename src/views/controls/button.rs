use crate::views::{View, ViewCoreInfo, ViewInfo};


pub struct Button {
    info: ViewInfo,

    text: String
}

impl Button {
    pub fn new(width: i32, height: i32) -> Button {
        Button { info: ViewInfo::Orphan((width, height)), text: String::new() }
    }

    pub fn get_text(&self) -> String { self.text.clone() }
    pub fn set_text(&mut self, text: String) { self.text = text; }
}

impl View for Button {
    fn reparent(&mut self, core_info: ViewCoreInfo) {
        self.info = ViewInfo::Child(core_info);
    }

    fn draw(&mut self, lx: i32, ly: i32, rx: i32, ry: i32) -> Vec<u8> {
        let width = rx - lx;
        let height = ry - ly;

        let mut buffers = Vec::new();
        for i in 0..width * height {
            buffers[i as usize] = 200;
        };

        buffers
    }
    
    fn get_info(&self) -> &ViewInfo {
        &self.info
    }
}