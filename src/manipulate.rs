use std::fmt::Display;

use image::{DynamicImage, GenericImageView, Rgba, RgbaImage};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArnoldCat {
    pub key: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Henon {
    pub key: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EncMethod {
    ArnoldCat(Option<ArnoldCat>),
    Henon(Option<Henon>),
}

impl Display for EncMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                EncMethod::ArnoldCat(_) => "ArnoldCat",
                EncMethod::Henon(_) => "Hénon",
            }
        )
    }
}

impl Default for EncMethod {
    fn default() -> Self {
        Self::ArnoldCat(None)
    }
}

impl EncMethod {
    pub fn encrypt(&self, images: Vec<DynamicImage>) -> Vec<DynamicImage> {
        let res: Vec<DynamicImage> = match self {
            EncMethod::ArnoldCat(enc) => images
                .into_iter()
                .map(|image| enc.clone().unwrap().encrypt(image))
                .collect(),
            EncMethod::Henon(enc) => images
                .into_iter()
                .map(|image| enc.clone().unwrap().encrypt(image))
                .collect(),
        };
        res
    }

    pub fn decrypt(&self, images: Vec<DynamicImage>) -> Vec<DynamicImage> {
        let res: Vec<DynamicImage> = match self {
            EncMethod::ArnoldCat(enc) => images
                .into_iter()
                .map(|image| enc.clone().unwrap().decrypt(image))
                .collect(),
            EncMethod::Henon(enc) => images
                .into_iter()
                .map(|image| enc.clone().unwrap().decrypt(image))
                .collect(),
        };
        res
    }
}

pub trait ImageEncyptor {
    fn encrypt(&self, image: DynamicImage) -> DynamicImage;
    fn decrypt(&self, image: DynamicImage) -> DynamicImage;
}

impl ImageEncyptor for ArnoldCat {
    fn encrypt(&self, image: DynamicImage) -> DynamicImage {
        let mut buffer = RgbaImage::new(image.width(), image.height());
        let rgba_image = image.to_rgba8();

        for x in 0..image.width() {
            for y in 0..image.height() {
                let nx = (2 * x * y) % image.width();
                let ny = (x + y) % image.height();

                buffer.put_pixel(
                    nx,
                    image.height() - ny - 1,
                    rgba_image.get_pixel(x, image.height() - y - 1).clone(),
                )
            }
        }

        DynamicImage::from(buffer)
    }

    fn decrypt(&self, image: DynamicImage) -> DynamicImage {
        image
    }
}

impl ImageEncyptor for Henon {
    fn encrypt(&self, image: DynamicImage) -> DynamicImage {
        image
    }

    fn decrypt(&self, image: DynamicImage) -> DynamicImage {
        image
    }
}
