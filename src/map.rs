use std::{
    io::{self},
    path::Path,
    sync::Arc,
};

use rfd::FileHandle;

#[derive(Debug, Clone)]
pub enum Error {
    DialogClosed,
    IO(io::ErrorKind),
}

pub async fn pick_and_load_images() -> Result<Arc<Vec<u8>>, Error> {
    let files = pick_files().await?;

    load_images(files.as_ref().iter().map(|f| f.path()).collect()).await
}

async fn load_images(paths: Vec<impl AsRef<Path>>) -> Result<Arc<Vec<u8>>, Error> {
    let raw = tokio::fs::read(&paths[0])
        .await
        .map(Arc::new)
        .map_err(|err| Error::IO(err.kind()));
    raw
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
