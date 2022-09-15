use crate::entity::crop::Crop;

#[derive(Clone)]
pub struct Region {
    pub left: u32,
    pub top: u32,
    pub width: u32,
    pub height: u32,
}

impl Region {
    pub fn from_size(width: u32, height: u32) -> Self {
        Self {
            left: 0,
            top: 0,
            width,
            height,
        }
    }

    pub fn crop(self: &Self, crop: &Crop) -> Result<Self, CropError> {
        let new_width = (self.width as f64 / crop.factor) * 100_f64;
        let region = Self {
            width: to_u32(new_width)?,
            height: to_u32(new_width)?,
            left: to_u32(crop.anchor.x * new_width * (-1.0))?,
            top: to_u32(crop.anchor.y * new_width * (-1.0))?,
        };

        if region.left + region.width > self.width || region.top + region.height > self.height {
            return Err(CropError::OutsideCrop);
        }

        Ok(region)
    }
}

#[derive(Debug)]
pub enum CropError {
    OutsideCrop,
}

fn to_u32(value: f64) -> std::result::Result<u32, CropError> {
    if value < 0.0 {
        Err(CropError::OutsideCrop)
    } else {
        Ok(value.round() as u32)
    }
}
