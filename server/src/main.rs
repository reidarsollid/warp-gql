mod graphql;
mod context;

use std::env;

use warp::{http::Response, Filter};
use crate::context::GraphQLContext;
use crate::graphql::schema::schema;

async fn get_alive() -> Result<impl warp::Reply, warp::Rejection> {
    Ok("Ok")
}

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "warp_server");
    env_logger::init();

    let log = warp::log("warp_server");

    let homepage = warp::path::end().map(|| {
        Response::builder()
            .header("content-type", "text/html")
            .body(format!(
                "<html><h1>juniper_warp</h1><div>visit <a href=\"/graphiql\">/graphiql</a></html>"
            ))
    });

    let alive = warp::get()
        .and(warp::path("alive"))
        .and(warp::path::end())
        .and_then(get_alive);

    log::info!("Listening on 127.0.0.1:8080");

    let state = warp::any().map(move || GraphQLContext {});
    let graphql_filter = juniper_warp::make_graphql_filter(schema(), state.boxed());
    warp::serve(
        warp::get()
            .and(warp::path("graphiql"))
            .and(juniper_warp::graphiql_filter("/graphql", None))
            .or(homepage)
            .or(warp::path("graphql").and(graphql_filter))
            .or(alive)
            .with(log),
    )
        .run(([127, 0, 0, 1], 8080))
        .await
}