extern crate thiserror;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_warp::Response;
use dotenv::dotenv;
use log::debug;
use std::convert::Infallible;
use structopt::StructOpt;
use warp::{http::Response as HttpResponse, Filter};

mod errors;
mod model;
mod opts;

use crate::model::{Configuration, Query, Storage};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let opt = opts::Opt::from_args();
    std::env::set_var("RUST_LOG", "trace");
    match opt.log_level {
        0 => std::env::set_var("RUST_LOG", "error"),
        1 => std::env::set_var("RUST_LOG", "warn"),
        2 => std::env::set_var("RUST_LOG", "info"),
        3 => std::env::set_var("RUST_LOG", "debug"),
        _ => std::env::set_var("RUST_LOG", "trace"),
    }
    pretty_env_logger::init();

    debug!("{:?}", opt);
    let config = Configuration::from_file(opt.config).await;
    debug!("{:?}", config);

    print!("Playground: http://localhost:8000/playground");

    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(config.clone())
        .data(Storage::new())
        .finish();

    let graphql_post = warp::path("query")
        .and(async_graphql_warp::graphql(schema))
        .and_then(
            |(schema, request): (
                Schema<Query, EmptyMutation, EmptySubscription>,
                async_graphql::Request,
            )| async move {
                Ok::<_, Infallible>(Response::from(schema.execute(request).await))
            },
        );

    let graphql_playground = warp::path("playground").map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/query")))
    });

    let static_files = warp::path("static").and(warp::fs::dir(opt.static_file_path));

    let routes = graphql_playground.or(static_files).or(graphql_post);

    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
