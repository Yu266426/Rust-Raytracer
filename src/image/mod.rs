pub mod color;

use std::{
    fs::{self},
    path::Path,
};

use color::Color;

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
