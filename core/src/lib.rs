use image::{DynamicImage, GenericImageView, Pixel, Rgba32FImage, RgbaImage};

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

impl ImageCipher for HyperChaosSVD {
    fn hash(key: String) -> String {
        key
    }

    fn encrypt(image: DynamicImage, key: String) -> DynamicImage {
        let dimensions = image.dimensions();

        let r_channel: Rgba32FImage = Rgba32FImage::from_vec(
            dimensions.0,
            dimensions.1,
            image
                .to_rgba32f()
                .enumerate_pixels()
                .map(|pix| pix.2.0[1])
                .collect(),
        )
        .unwrap();

        println!("{image:#?}");
        println!("{r_channel:#?}");

        todo!()
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
