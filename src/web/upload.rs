use crate::entity::picture::{Entity, Picture};
use poem::{
    handler,
    http::StatusCode,
    web::{headers::ContentType, Data, Multipart},
    Response,
};
use sqlx::SqlitePool;

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
            log::debug!("name={name:?} filename={file_name} length={}", bytes.len());

            let pic = Picture::create(&bytes, &cfg.images_folder).unwrap();

            Entity::from(&pic).insert_or_ignore(db).await.unwrap();

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
