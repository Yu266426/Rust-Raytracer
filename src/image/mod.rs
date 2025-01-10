pub mod color;

use std::{
    fs::{self},
    path::Path,
};

use color::Color;
use image::{open, RgbImage};

pub struct Image {
    width: usize,
    height: usize,
    image_data: Vec<Color>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        let mut image_data = Vec::with_capacity(width * height);

        for _ in 0..(width * height) {
            image_data.push(Color::black());
        }

        Self {
            width,
            height,
            image_data,
        }
    }

    pub fn save(&self, name: &str) {
        let file_name = format!("{}.ppm", name);
        let path = Path::new(&file_name);

        let mut data = String::new();
        data += "P3\n";
        data += format!("{} {}\n", self.width, self.height).as_str();
        data += "255\n";
        self.image_data.iter().for_each(|e| {
            data += e.as_u8_string().as_str();
            data += " ";
        });

        fs::write(path, data).expect("Unable to write to file");
    }

    pub fn set_pixel(&mut self, color: Color, row: usize, col: usize) {
        let index = row * self.width + col;

        self.image_data[index] = color;
    }
}

pub struct ExtImage {
    image: RgbImage,
    width: usize,
    height: usize,
}

impl ExtImage {
    pub fn load(file_name: &str) -> Result<Self, String> {
        let path = Path::new(file_name);

        if !path.is_file() {
            return Err(format!("File `{}` not found", file_name));
        }

        let image = match open(path) {
            Ok(image) => image.to_rgb8(),
            Err(_) => return Err(format!("Could not load `{}`", path.to_str().unwrap())),
        };

        let width = image.width() as usize;
        let height = image.height() as usize;

        Ok(Self {
            image,
            width,
            height,
        })
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn pixel_data(&self, x: usize, y: usize) -> &[u8; 3] {
        static MAGENTA: [u8; 3] = [255, 0, 255];
        if false {
            // Debug if image doesn't load (not sure if needed)
            return &MAGENTA;
        }

        let x = x.clamp(0, self.width) as u32;
        let y = y.clamp(0, self.height) as u32;

        &self.image.get_pixel(x, y).0
    }
}
