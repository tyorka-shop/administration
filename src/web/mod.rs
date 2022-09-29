mod gql;
mod upload;

use poem::{listener::TcpListener, post, EndpointExt, Route, Server, middleware::Cors, endpoint::StaticFilesEndpoint};
use sqlx::SqlitePool;
use std::{future::Future, net::SocketAddr, str::FromStr};

use crate::{graphql_schema::build_schema, guard::RoleExctractor, image_storage::ImageStorage};

pub async fn make_server(
    cfg: config::Config,
    db: SqlitePool,
    images: ImageStorage,
) -> impl Future<Output = Result<(), std::io::Error>> {
    let schema = build_schema().finish();

    let addr = SocketAddr::from_str(&cfg.addr).unwrap();

    let extractor = RoleExctractor::new(&cfg.secret, "http://localhost:50051".into());
    let builder = crate::builder::Builder::new(&cfg.public_site_folder);

    let app = Route::new()
        .nest("/static/images", StaticFilesEndpoint::new(&cfg.images_folder))
        .at("/graphql", post(gql::handler))
        .at("/upload", post(upload::handler))
        .with(Cors::new().allow_origins(cfg.cors_allowed_origins.clone()).allow_credentials(true))
        .data(schema)
        .data(extractor)
        .data(db)
        .data(cfg)
        .data(images)
        .data(builder);

    log::info!("GraphQL listening on {}", addr);
    Server::new(TcpListener::bind(addr)).run(app)
}
