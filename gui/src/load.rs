use std::{io::Cursor, path::Path, sync::Arc};

use image::{io::Reader, DynamicImage};
use rfd::FileHandle;

use crate::Error;

pub async fn pick_and_load_images() -> Result<Arc<Vec<DynamicImage>>, Error> {
    let files = pick_files().await?;

    load_images(
        files
            .as_ref()
            .iter()
            .map(|file_handle| file_handle.path())
            .collect(),
    )
    .await
}

async fn load_images(paths: Vec<impl AsRef<Path>>) -> Result<Arc<Vec<DynamicImage>>, Error> {
    let mut images = Vec::new();
    for path in paths {
        let raw = tokio::fs::read(path)
            .await
            .map_err(|err| Error::IO(err.kind()))?;

        let image = Reader::new(Cursor::new(raw))
            .with_guessed_format()
            .map_err(|err| Error::IO(err.kind()))?
            .decode()
            .map_err(|_| Error::ImageDecode)?;

        images.push(image);
    }

    Ok(Arc::new(images))
}

async fn pick_files() -> Result<Arc<Vec<FileHandle>>, Error> {
    rfd::AsyncFileDialog::new()
        .set_title("Choose images")
        .add_filter("Image", &["png", "jpg", "jpeg", "webp"])
        .pick_files()
        .await
        .map(Arc::new)
        .ok_or(Error::DialogClosed)
}
