use image::DynamicImage;

pub trait ImageEncyptor {
    fn encrypt(image: DynamicImage) -> DynamicImage;
    fn decrypt(image: DynamicImage) -> DynamicImage;
}
