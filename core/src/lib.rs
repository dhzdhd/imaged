use image::{
    DynamicImage, GenericImageView, Pixel, Rgb32FImage, RgbImage, Rgba, Rgba32FImage, RgbaImage,
};

pub trait ImageCipher {
    fn hash(key: String) -> String;
    fn encrypt(image: DynamicImage, key: String) -> DynamicImage;
    fn decrypt(image: DynamicImage, key: String) -> DynamicImage;
}

pub enum CipherMethod {
    ArnoldCat(ArnoldCat),
    HenonMap(HenonMap),
    HyperChaosSVD(HyperChaosSVD),
}

pub struct ArnoldCat;
pub struct HenonMap;
pub struct HyperChaosSVD;

fn chaotic_sequence_generation(image: RgbaImage) {
    let f = 2.7;
    let g = -3;
    let h = 10;
    let k = 2;
    let l = 100;
    let m = 1;

    let s_chaotic = 2;
}

impl ImageCipher for HyperChaosSVD {
    fn hash(key: String) -> String {
        key
    }

    fn encrypt(image: DynamicImage, key: String) -> DynamicImage {
        let dimensions = image.dimensions();
        let rgba_image = image.to_rgba8();

        let red_channel = RgbaImage::from_fn(dimensions.0, dimensions.1, |x, y| {
            let pixel = rgba_image.get_pixel(x, y);
            Rgba([pixel[0], 0, 0, pixel[3]])
        });
        let green_channel = RgbaImage::from_fn(dimensions.0, dimensions.1, |x, y| {
            let pixel = rgba_image.get_pixel(x, y);
            Rgba([0, pixel[1], 0, pixel[3]])
        });
        let blue_channel = RgbaImage::from_fn(dimensions.0, dimensions.1, |x, y| {
            let pixel = rgba_image.get_pixel(x, y);
            Rgba([0, 0, pixel[2], pixel[3]])
        });

        red_channel.into()
    }

    fn decrypt(image: DynamicImage, key: String) -> DynamicImage {
        image
    }
}

impl ImageCipher for ArnoldCat {
    fn hash(key: String) -> String {
        key
    }
    fn encrypt(image: DynamicImage, key: String) -> DynamicImage {
        image
    }
    fn decrypt(image: DynamicImage, key: String) -> DynamicImage {
        image
    }
}
