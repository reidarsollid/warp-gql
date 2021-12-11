use std::env;
use std::sync::Arc;

use juniper::{EmptyMutation, EmptySubscription, RootNode, FieldResult, GraphQLObject, Context};
use warp::{http::Response, Filter};


pub struct QueryRoot;

#[derive(Clone, Copy, Debug)]
pub struct GraphQLContext {}

impl juniper::Context for GraphQLContext {}

#[derive(GraphQLObject)]
//#[graphql(description = "User preferences for content", name = "UserPreferences")]
struct Person {
    name: String,
    age: i32,
}

#[juniper::graphql_object(Context = GraphQLContext)]
impl QueryRoot {
    async fn person(context: &GraphQLContext) -> FieldResult<Person> {
        Ok(Person {
            name: "Reidar".to_string(),
            age: 51,
        })
    }
}

type Schema = RootNode<'static, QueryRoot, EmptyMutation<GraphQLContext>, EmptySubscription<GraphQLContext>>;

fn schema() -> Schema {
    Schema::new(
        QueryRoot,
        EmptyMutation::<GraphQLContext>::new(),
        EmptySubscription::<GraphQLContext>::new(),
    )
}

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

    //let app_state = warp::any().map(move || app_state.clone());
    let ctx = GraphQLContext {};
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