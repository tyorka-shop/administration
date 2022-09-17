use image_processing::{Crop, Image};

use crate::graphql_types::Picture;

#[derive(Debug, Clone)]
pub struct ImageStorage {
    pub folder: String,
    pub sizes: Vec<u32>,
}

impl ImageStorage {
    pub fn new(folder: &str, sizes: Vec<u32>) -> std::io::Result<Self> {
        match std::fs::create_dir_all(folder) {
            Ok(_) => {}
            Err(e) => match e.kind() {
                std::io::ErrorKind::AlreadyExists => {}
                _ => {
                    return Err(e);
                }
            },
        };

        Ok(Self {
            folder: folder.to_string(),
            sizes,
        })
    }

    pub fn create(&self, bytes: &Vec<u8>) -> std::io::Result<Picture> {
        let img = Image::new(&bytes).unwrap();

        let (w, h) = img.size();
        let crop = Crop::default_square(w, h);

        img.save(&self.folder, &self.sizes).unwrap();

        Ok(Picture::new(
            &img.id(),
            w as i64,
            h as i64,
            img.dominant_color().as_ref(),
            &crop.into(),
        ))
    }

    pub fn recrop(&self, id: &str, crop: &Crop) -> std::io::Result<()> {
        let img = image_processing::Image::from_file(&self.folder, &id).unwrap();

        img.recrop(&self.folder, self.sizes.clone(), &crop.clone().into())
            .unwrap();
        Ok(())
    }
}
