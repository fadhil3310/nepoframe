use crate::views::{View, ViewCore, ViewHierarchical, ViewInfo};


pub struct Row {
    core: ViewInfo,

    children: Vec<Box<dyn View>>
}

impl Row {
    fn resize_child(&self, view: &dyn View, size: (i32, i32)) -> Option<(i32, i32)> {
        Some(size)
    }
}

impl ViewHierarchical for Row {
    fn add_child(&'static mut self, view: Box<dyn View>) {
        let view_size: (i32, i32);
        let view_position: (i32, i32);

        match view.get_info() {
            ViewInfo::Orphan(info) => {
                view_size = info.clone();
                view_position = (0, 0);
            }
            ViewInfo::Child(info) => {
                view_size = info.get_size();
                view_position = info.get_position();
            }
        }

        let core = ViewCore {
            parent: self,
           
            x: view_position.0,
            y: view_position.1,
            width: view_size.0,
            height: view_size.1,

            request_set_position: Box::new(|pos| { None }),
            request_set_size: Box::new(|size| { self.resize_child(view.as_ref(), size) })
        };

        view.reparent(core);

        self.children.push(view);
    }
}