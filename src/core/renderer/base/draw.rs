use super::texture::Texture;

pub trait Draw {
  fn draw_surface(&self, x: i32, y: i32, width: i32, height: i32, texture: Texture);
}
