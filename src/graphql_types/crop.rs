use async_graphql::{InputObject, SimpleObject};
use image_processing;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, SimpleObject, InputObject)]
#[graphql(input_name = "PointInput")]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Default for Point {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

#[derive(Clone, Debug, Serialize, SimpleObject, InputObject)]
#[graphql(input_name = "CropInput")]
pub struct Crop {
    pub anchor: Point,

    pub factor: f64,
}

impl Default for Crop {
    fn default() -> Self {
        Self {
            anchor: Point::default(),
            factor: 100.0,
        }
    }
}

impl From<image_processing::Crop> for Crop {
    fn from(crop: image_processing::Crop) -> Self {
        Self {
            anchor: Point {
                x: crop.anchor.x,
                y: crop.anchor.y,
            },
            factor: crop.factor,
        }
    }
}

impl From<Crop> for image_processing::Crop {
    fn from(crop: Crop) -> Self {
        Self {
            anchor: image_processing::Point {
                x: crop.anchor.x,
                y: crop.anchor.y,
            },
            factor: crop.factor,
        }
    }
}
