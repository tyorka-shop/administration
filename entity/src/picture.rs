#[derive(macros::Entity)]
#[table_name = "pictures"]
pub struct Picture {
    pub id: String,
    pub color: String,
    pub original_size_width: i64,
    pub original_size_height: i64,
    pub crop_anchor_x: f64,
    pub crop_anchor_y: f64,
    pub crop_factor: f64,
    pub product_id: Option<String>,
    pub idx: Option<i64>,
}

impl Picture {
  pub fn new_fixture() -> Self {
      Self {
          id: "4e2d05fa-d79c-401c-8cdf-275eb2dccbae".into(),
          color: "#000000".into(),
          original_size_width: 100,
          original_size_height: 100,
          crop_anchor_x: 0.5,
          crop_anchor_y: 0.5,
          crop_factor: 1.0,
          product_id: None,
          idx: None,
      }
  }
}