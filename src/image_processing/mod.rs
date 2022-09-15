mod crop;
mod image;

use crate::entity::picture::Picture;
use crate::{entity::crop::Crop, image_processing::image::Image};
use rayon::prelude::*;
use std::path::Path;

const SIZES: &[u32] = &[200, 600, 2000];
const EXT: &str = "jpg";

impl Picture {
    pub fn create(bytes: &[u8], path: &str) -> std::io::Result<Self> {
        let img = Image::new(bytes).unwrap();

        let id = img.get_id();
        let (w, h) = img.size();
        let dominant_color = img.dominant_color();
        let pic = Self::new(&id, w.into(), h.into(), &dominant_color);

        log::debug!("Processing file {:?}", &pic);

        let filename = format!("{path}/{id}.{EXT}");

        if Path::new(&filename).exists() {
            log::debug!("File {} already exists", filename);
            return Ok(pic);
        }

        img.save(&filename).unwrap();

        SIZES
            .par_iter()
            .map(|height| {
                let mut img = img.clone();
                img.resize(*height);

                img.save(&format!("{path}/{id}_{height}.{EXT}")).unwrap();

                let (w, h) = img.size();

                let crop = Crop::default_square(w, h);

                img.crop(&crop).unwrap();

                img.save(&format!("{path}/{id}_square_{height}.{EXT}"))
                    .unwrap();
            })
            .collect::<Vec<_>>();

        Ok(pic)
    }
}

pub fn init_store(path: &str) -> std::io::Result<()> {
    match std::fs::create_dir_all(path) {
        Ok(_) => Ok(()),
        Err(e) => match e.kind() {
            std::io::ErrorKind::AlreadyExists => Ok(()),
            _ => Err(e),
        },
    }
}

// pub fn save_crop(bytes: &[u8], path: &str, filename: &str, crop: &Crop) -> Result<(), CropError> {
//     let (name, ext) = make_filename(&bytes, filename);

//     let mut img = image::load_from_memory(bytes).unwrap();

//     let region = Region::from_size(img.width(), img.height())
//         .from_crop(crop)
//         .unwrap();

//     let cropped = img.crop(region.left, region.top, region.width, region.height);

//     let resized = resize(&cropped, 600);

//     let filename = format!("{}/{}_squared_600.{}", path, &name, &ext);
//     resized.save(&filename).unwrap();
//     log::debug!("Cropped saved {}", &filename);

//     Ok(())
// }
