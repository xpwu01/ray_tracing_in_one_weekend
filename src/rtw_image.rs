use image::ImageError;
use std::env;
use std::path::{Path, PathBuf};

pub struct RtwImage {
    bytes_per_pixel: usize,
    width: u32,
    height: u32,
    float_data: Option<Vec<f32>>,
    byte_data: Option<Vec<u8>>,
}

impl RtwImage {
    pub fn new(image_filename: &str) -> Result<Self, String> {
        let mut image = RtwImage {
            bytes_per_pixel: 3,
            width: 0,
            height: 0,
            float_data: None,
            byte_data: None,
        };

        let image_dir = env::var("RTW_IMAGES").ok();

        // List of potential paths to search for the image
        let search_paths = vec![
            image_dir.map(|dir| Path::new(&dir).join(image_filename)),
            Some(PathBuf::from(image_filename)),
            Some(Path::new("images").join(image_filename)),
            Some(Path::new("../images").join(image_filename)),
            Some(Path::new("../../images").join(image_filename)),
            Some(Path::new("../../../images").join(image_filename)),
            Some(Path::new("../../../../images").join(image_filename)),
            Some(Path::new("../../../../../images").join(image_filename)),
            Some(Path::new("../../../../../../images").join(image_filename)),
        ];

        for path in search_paths.into_iter().flatten() {
            if let Ok(loaded_image) = image.load(&path) {
                return Ok(loaded_image);
            }
        }

        Err(format!(
            "ERROR: Could not load image file '{}'.",
            image_filename
        ))
    }

    fn load(&mut self, filename: &Path) -> Result<Self, ImageError> {
        let img = image::open(filename)?;
        self.width = img.width();
        self.height = img.height();

        // Convert image to RGB and store as 8-bit pixel data
        let rgb_image = img.to_rgb8();
        self.byte_data = Some(
            rgb_image
                .clone()
                .into_raw()
                .iter_mut()
                .map(|byte| ((*byte as f64).powi(2) / 255.0) as u8)
                .collect(),
        );

        // Convert 8-bit pixel data to floating-point data (linear gamma)
        self.float_data = Some(
            rgb_image
                .pixels()
                .flat_map(|pixel| pixel.0.iter().map(|&byte| (byte as f32 / 255.0).powi(2)))
                .collect(),
        );

        Ok(self.clone())
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn pixel_data(&self, x: i32, y: i32) -> &[u8] {
        static MAGENTA: [u8; 3] = [255, 0, 255];
        if self.byte_data.is_none() {
            return &MAGENTA;
        }

        let x = Self::clamp(x, 0, self.width as i32);
        let y = Self::clamp(y, 0, self.height as i32);

        let index = (y as usize * self.width as usize + x as usize) * self.bytes_per_pixel;
        &self.byte_data.as_ref().unwrap()[index..index + self.bytes_per_pixel]
    }

    fn clamp(value: i32, min: i32, max: i32) -> i32 {
        value.clamp(min, max - 1)
    }
}

impl Clone for RtwImage {
    fn clone(&self) -> Self {
        RtwImage {
            bytes_per_pixel: self.bytes_per_pixel,
            width: self.width,
            height: self.height,
            float_data: self.float_data.clone(),
            byte_data: self.byte_data.clone(),
        }
    }
}
