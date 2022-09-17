use crate::graphql_types::{Crop, Picture};
use image_processing::Image;
use poem::{
    handler,
    http::StatusCode,
    web::{headers::ContentType, Data, Multipart},
    Response,
};
use sqlx::SqlitePool;

const SIZES: &[u32] = &[200, 600, 2000];

#[handler]
pub async fn handler(
    Data(cfg): Data<&config::Config>,
    Data(db): Data<&SqlitePool>,
    mut multipart: Multipart,
) -> Response {
    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().map(ToString::to_string);
        let file_name = match field.file_name().map(ToString::to_string) {
            Some(name) => name,
            None => panic!("Uploading file has no name"),
        };

        if let Ok(bytes) = field.bytes().await {
            log::debug!(
                "Uploaded name={name:?} filename={file_name} length={}",
                bytes.len()
            );

            let img = Image::new(&bytes).unwrap();

            let (w, h) = img.size();
            let crop: Crop = image_processing::Crop::default_square(w, h).into();

            img.save(&cfg.images_folder, SIZES.into()).unwrap();

            let (w, h) = img.size();

            let pic = Picture::new(
                &img.id(),
                w as i64,
                h as i64,
                img.dominant_color().as_ref(),
                &crop,
            );

            entity::Picture::from(&pic)
                .insert_or_ignore(db)
                .await
                .unwrap();

            let content_type: String = ContentType::json().to_string();

            return Response::builder()
                .content_type(&content_type)
                .body(serde_json::to_string(&pic).unwrap());
        }
    }

    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body("Expected file")
}
