mod gql;
mod upload;

use poem::{listener::TcpListener, post, EndpointExt, Route, Server};
use sqlx::SqlitePool;
use std::{
    future::Future,
    net::{Ipv4Addr, SocketAddr},
};

use crate::{graphql_schema::build_schema, guard::RoleExctractor};

pub async fn make_server(
    cfg: config::Config,
    db: SqlitePool,
) -> impl Future<Output = Result<(), std::io::Error>> {
    let schema = build_schema().finish();

    let port = cfg.port.parse::<u16>().unwrap();
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), port);

    let extractor = RoleExctractor::new(&cfg.secret, "http://localhost:50051".into());

    let app = Route::new()
        .at("/graphql", post(gql::handler))
        .at("/upload", post(upload::handler))
        .data(schema)
        .data(extractor)
        .data(db)
        .data(cfg);

    log::info!("GraphQL listening on {}", port);
    Server::new(TcpListener::bind(addr)).run(app)
}
