//! Types and declarations for the Object Attribute Memory (`OAM`).

newtype! {
  // TODO
  #[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
  OBJAttr0, u16
}
impl OBJAttr0 {
  bool_bits!(
    u16,
    [
      (12, mosaic),
      (13, is_8bpp),
    ]
  );

  multi_bits!(
    u16,
    [
      (0, 8, y_coordinate),
      (8, 2, obj_rendering, ObjectRender, Normal, Affine, Disabled, DoubleAreaAffine),
      (10, 2, obj_mode, ObjectMode, Normal, SemiTransparent, OBJWindow),
      (14, 2, obj_shape, ObjectShape, Square, Horizontal, Vertical),
    ]
  );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ObjectRender {
  Normal = 0,
  Affine = 1,
  Disabled = 2,
  DoubleAreaAffine = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ObjectMode {
  Normal = 0,
  SemiTransparent = 1,
  OBJWindow = 2
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ObjectShape {
  Square = 0,
  Horizontal = 1,
  Vertical = 2
}

newtype! {
  // TODO
  #[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
  OBJAttr1, u16
}
impl OBJAttr1 {
  bool_bits!(
    u16,
    [
      (12, hflip),
      (13, vflip),
    ]
  );

  multi_bits!(
    u16,
    [
      (0, 9, x_coordinate),
      (9, 5, affine_index),
      (14, 2, obj_size, ObjectSize, Zero, One, Two, Three),
    ]
  );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ObjectSize {
  Zero = 0,
  One = 1,
  Two = 2,
  Three = 3,
}

newtype! {
  // TODO
  #[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
  OBJAttr2, u16
}
impl OBJAttr2 {
  multi_bits!(
    u16,
    [
      (0, 10, tile_id),
      (10, 2, priority),
      (12, 4, palbank),
    ]
  );
}
