pub enum TextureFillMode {
  Fit,
  Stretch,
  Cover,
}

pub struct Texture<'a> {
  buffer: &'a [i32],
  fill_mode: TextureFillMode,
}
