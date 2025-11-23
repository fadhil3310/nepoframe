
pub trait View {
    fn draw(&mut self, lx: i32, ly: i32, rx: i32, ry: i32) -> Vec<u8>;
    
    fn reparent(&mut self, core: ViewCore);

    fn get_info(&self) -> &ViewInfo;
}

pub trait ViewHierarchical {
    fn add_child(&'static mut self, view: Box<dyn View>);
}


pub struct ViewCore {
    parent: &'static dyn ViewHierarchical,

    x: i32,
    y: i32,
    width: i32,
    height: i32,

    pub request_set_position: Box<dyn Fn((i32, i32)) -> Option<(i32, i32)>>,
    pub request_set_size: Box<dyn Fn((i32, i32)) -> Option<(i32, i32)>>,
}

impl ViewCore {
    pub fn get_parent(&self) -> &'static dyn ViewHierarchical { self.parent }

    pub fn get_position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
    pub fn set_position(&mut self, pos: (i32, i32)) -> bool {
        if let Some(result_pos) = (self.request_set_position)(pos) {
            self.x = result_pos.0;
            self.y = result_pos.1;
            true
        } else {
            false
        }
    }
    pub fn get_size(&self) -> (i32, i32) { (self.width, self.height) }
    pub fn set_size(&mut self, size: (i32, i32)) -> bool {
        if let Some(result_size) = (self.request_set_size)(size) {
            self.width = result_size.0;
            self.height = result_size.1;
            true
        } else {
            false
        }
    }
}


// pub struct ViewCoreInfo {
//     pub get_parent: fn() -> &'static dyn ViewHierarchical,

//     pub get_position: fn() -> (i32, i32),
//     pub set_position: fn(val: (i32, i32)) -> bool,
//     pub get_size: fn() -> (i32, i32),
//     pub set_size: fn(val: (i32, i32)) -> bool,
// }


pub enum ViewInfo {
    Orphan((i32, i32)),
    Child(ViewCore)
}


pub mod controls;
pub mod layouts;