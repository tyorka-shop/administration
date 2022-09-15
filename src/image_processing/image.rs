use crate::entity::crop;

use super::crop::{CropError, Region};
use image::{imageops::FilterType::Lanczos3, DynamicImage, ImageOutputFormat};
use std::{fs::File, result::Result};

#[derive(Clone)]
pub struct Image {
    img: DynamicImage,
    filename: String,
}

impl Image {
    pub fn new(bytes: &[u8]) -> std::io::Result<Self> {
        let img = image::load_from_memory(bytes).unwrap();

        Ok(Self {
            img,
            filename: format!("{:x}", md5::compute(bytes)),
        })
    }

    pub fn size(&self) -> (u32, u32) {
        (self.img.width(), self.img.height())
    }

    pub fn resize(self: &mut Self, new_height: u32) -> () {
        log::debug!("Resizing image {} to height={}", &self.filename, new_height);
        let new_width =
            (new_height as f32 * self.img.width() as f32 / self.img.height() as f32) as u32;

        self.img = self.img.resize_exact(new_width, new_height, Lanczos3);
    }

    pub fn crop(self: &mut Self, crop: &crop::Crop) -> Result<(), CropError> {
        log::debug!("Crop image {} to {:?}", &self.filename, crop);
        let region = Region::from_size(self.img.width(), self.img.height()).crop(crop)?;

        let mut img = self.img.clone();

        self.img = img.crop(region.left, region.top, region.width, region.height);
        Ok(())
    }

    pub fn get_id(&self) -> String {
        self.filename.clone()
    }

    pub fn save(&self, filename: &str) -> std::io::Result<()> {
        log::debug!("Saving image to {}", filename);
        let mut file = match File::create(&filename) {
            Ok(file) => file,
            Err(e) => match e.kind() {
                std::io::ErrorKind::AlreadyExists => {
                    return Ok(());
                }
                _ => return Err(e),
            },
        };
        self.img
            .write_to(&mut file, ImageOutputFormat::Jpeg(90))
            .unwrap();
        Ok(())
    }

    pub fn dominant_color(&self) -> String {
        let colors = dominant_color::get_colors(&self.img.as_bytes().to_vec(), false);
        // colors[0..2].iter().map(|c| format!("{:02x}", c)).collect::<Vec<String>>().join("")
        format!("#{:02x}{:02x}{:02x}", colors[0], colors[1], colors[2])
    }
}
