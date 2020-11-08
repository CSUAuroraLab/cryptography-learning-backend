extern crate thiserror;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use dotenv::dotenv;
use log::{debug, error, info, warn};
use structopt::StructOpt;
use async_graphql_warp::{BadRequest, Response};
use http::StatusCode;
use std::convert::Infallible;
use warp::{http::Response as HttpResponse, Filter, Rejection};


mod errors;
mod model;
mod opts;

use crate::model::{Configuration, Query};

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
    env_logger::init();
    debug!("{:?}", opt);
    let static_file_path = opt.static_file_path;

    let config = Configuration::from_file(opt.config);
    debug!("{:?}", config);

    print!("Playground: http://localhost:8000/");

    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(config.clone())
        .finish();

    println!("Playground: http://localhost:8000");

    let graphql_post = async_graphql_warp::graphql(schema).and_then(
        |(schema, request): (
            Schema<Query, EmptyMutation, EmptySubscription>,
            async_graphql::Request,
        )| async move { Ok::<_, Infallible>(Response::from(schema.execute(request).await)) },
    );

    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/")))
    });

    let routes = graphql_playground
        .or(graphql_post)
        .recover(|err: Rejection| async move {
            if let Some(BadRequest(err)) = err.find() {
                return Ok::<_, Infallible>(warp::reply::with_status(
                    err.to_string(),
                    StatusCode::BAD_REQUEST,
                ));
            }

            Ok(warp::reply::with_status(
                "INTERNAL_SERVER_ERROR".to_string(),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        });

    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
