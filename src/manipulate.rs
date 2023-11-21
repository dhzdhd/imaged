use std::fmt::Display;

use image::DynamicImage;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArnoldCat {
    key: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EncMethod {
    ArnoldCat(Option<ArnoldCat>),
    Henon,
}

impl Display for EncMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                EncMethod::ArnoldCat(_) => "ArnoldCat",
                EncMethod::Henon => "Hénon",
            }
        )
    }
}

impl Default for EncMethod {
    fn default() -> Self {
        Self::ArnoldCat(None)
    }
}

pub trait ImageEncyptor {
    fn encrypt(image: DynamicImage) -> DynamicImage;
    fn decrypt(image: DynamicImage) -> DynamicImage;
}
