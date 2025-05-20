use image::DynamicImage;

pub trait ImageCipher {
    fn hash(key: String) -> String;
    fn encrypt(image: DynamicImage, key: String) -> DynamicImage;
    fn decrypt(image: DynamicImage, key: String) -> DynamicImage;
}

pub struct ArnoldCat;
pub struct HenonMap;
pub struct HyperChaosSVD;

impl ImageCipher for ArnoldCat {
    fn hash(key: String) -> String {
        key
    }

    fn encrypt(image: DynamicImage, key: String) -> DynamicImage {
        let r = &image;
        image
    }

    fn decrypt(image: DynamicImage, key: String) -> DynamicImage {
        image
    }
}

impl ImageCipher for HyperChaosSVD {
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
