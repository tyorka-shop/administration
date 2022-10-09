use crate::{
    builder::Builder, graphql_schema::Schema, guard::RoleExctractor, image_storage::ImageStorage,
    publication_status::PublicationStatus,
};
use async_graphql_poem::{GraphQLRequest, GraphQLResponse};
use poem::{handler, http::HeaderMap, web::Data};
use sqlx::SqlitePool;

#[handler]
pub async fn handler(
    schema: Data<&Schema>,
    Data(db): Data<&SqlitePool>,
    Data(images): Data<&ImageStorage>,
    Data(role_extractor): Data<&RoleExctractor>,
    Data(builder): Data<&Builder>,
    Data(cfg): Data<&config::Config>,
    Data(publication_status): Data<&PublicationStatus>,
    headers: &HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let role = role_extractor.extract(headers).await;

    let req = req.0;

    log::debug!("req: {:?}", req);

    let req = req
        .data(role)
        .data(cfg.clone())
        .data(db.clone())
        .data(images.clone())
        .data(builder.clone())
        .data(publication_status.clone());
    let resp: GraphQLResponse = schema.execute(req).await.into();
    log::debug!("resp: {:?}", resp.0);
    resp
}
