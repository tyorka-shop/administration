
use crate::Crop;

use super::region::{CropError, Region};
use image::{imageops::FilterType::Lanczos3, DynamicImage, ImageOutputFormat};
use rayon::prelude::*;
use std::path::Path;
use std::{fs::File, result::Result};

const EXT: &str = "jpg";

#[derive(Clone)]
pub struct Image {
    id: String,
    img: DynamicImage,
}

impl Image {
    pub fn new(bytes: &[u8]) -> std::io::Result<Self> {
        let img = image::load_from_memory(bytes).unwrap();

        Ok(Self {
            id: format!("{:x}", md5::compute(bytes)),
            img,
        })
    }

    pub fn from_file(path: &str, id: &str) -> std::io::Result<Self> {
        let filename = format!("{}/{}.jpg", &path, &id);
        let img = image::open(&filename).unwrap();

        Ok(Self {
            id: id.to_string(),
            img,
        })
    }

    pub fn size(&self) -> (u32, u32) {
        (self.img.width(), self.img.height())
    }

    pub fn resize(self: &mut Self, new_height: u32) -> () {
        log::debug!("Resizing image {} to height={}", &self.id, new_height);
        let new_width =
            (new_height as f32 * self.img.width() as f32 / self.img.height() as f32) as u32;

        self.img = self.img.resize_exact(new_width, new_height, Lanczos3);
    }

    pub fn crop(self: &mut Self, region: &Region) -> Result<(), CropError> {
        log::debug!("Crop image {} to {:?}", &self.id, &region);

        let mut img = self.img.clone();

        self.img = img.crop(region.left, region.top, region.width, region.height);
        Ok(())
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    fn do_save(&self, filename: &str) -> std::io::Result<()> {
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

    fn save_variants(&self, path: &str, sizes: &[u32], crop: &Crop) -> std::io::Result<()> {
        sizes
            .par_iter()
            .map(move |height| {
                let mut variant = self.clone();

                variant.resize(*height);

                variant.do_save(&format!("{path}/{}_{height}.{EXT}", &self.id)).unwrap();

                let (w, h) = variant.size();

                let region = Region::from_size(w, h).crop(crop).unwrap();

                variant.crop(&region).unwrap();

                variant.do_save(&format!("{path}/{}_square_{height}.{EXT}", &self.id))
                    .unwrap();
            })
            .collect::<Vec<_>>();

        Ok(())
    }

    pub fn save(self: &Self, path: &str, sizes: &Vec<u32>) -> std::io::Result<()> {
        let filename = format!("{path}/{}.{EXT}", &self.id);

        log::debug!("Processing file {:?}", &filename);

        if Path::new(&filename).exists() {
            log::debug!("File {} already exists", filename);
            return Ok(());
        }

        self.do_save(&filename).unwrap();
        
        let (w, h) = self.size();

        let center = Crop::default_square(w, h);

        self.save_variants(path, &sizes, &center).unwrap();

        Ok(())
    }

    pub fn recrop(&self, path: &str, sizes: Vec<u32>, crop: &Crop) -> std::io::Result<()> {
        self.save_variants(path, &sizes, &crop).unwrap();

        Ok(())
    }
}
