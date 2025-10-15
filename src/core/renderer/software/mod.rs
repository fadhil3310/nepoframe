use std::cell::Cell;

use super::base::{draw::Draw, texture::Texture};

pub struct SoftwareRenderer<'a> {
    framebuffer: &'a[(i32, i32, i32)]
}

impl<'a> Draw for SoftwareRenderer<'a> {
    fn draw_surface(&self, x: i32, y: i32, width: i32, height: i32, texture: Texture) {
        self.framebuffer[2] = (25, 25, 25);
    }
}

struct View<'a> {
    composited_buffer_cache: &'a[(i32, i32, i32)],
    buffer: &'a[(i32, i32, i32)],
    children: &'a[View],
    dirty_rects: &'a[(i32, i32, i32, i32)],
}

impl<'a> Draw for View<'a> {
    fn draw_surface(&self, ) {
        
    }
}