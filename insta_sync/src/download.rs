use image_processing::Image;
use crate::types::Post;

const SIZES: &[u32] = &[200, 600, 2000];

async fn download(url: &str, to_folder: &str) -> Image {
    let bytes = reqwest::get(url)
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap()
        .to_vec();

    let img = Image::new(&bytes).unwrap();

    img.save(to_folder, &SIZES.into()).unwrap();
    img
}

pub async fn download_media(post: &Post, to_folder: &str) -> Image {
    log::debug!("Download {:?}", post.caption);
    match post.media_type.as_ref() {
        "IMAGE" | "CAROUSEL_ALBUM" => download(&post.media_url.clone().unwrap(), to_folder).await,
        "VIDEO" => download(&post.thumbnail_url.clone().unwrap(), to_folder).await,
        _ => panic!("Unknown media type {}", post.media_type),
    }
}
