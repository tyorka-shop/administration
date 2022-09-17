use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Default for Point {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}


#[derive(Clone, Debug, Serialize)]
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

impl Crop {
    pub fn default_square(width: u32, height: u32) -> Self {
        if width <= height || height == 0 {
            Self::default()
        } else {
            let ratio = width as f64 / height as f64;
            Self {
                anchor: Point {
                    x: (1.0 - ratio) / 2.0,
                    y: 0.0,
                },
                factor: 100.0 * ratio,
            }
        }
    }
}