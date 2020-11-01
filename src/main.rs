extern crate thiserror;

use actix_web::{guard, web, App, HttpResponse, HttpServer};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{Request, Response};
use log::{info, warn, error, debug};
use structopt::StructOpt;
use dotenv::dotenv;

mod model;
mod opts;
mod errors;

use crate::model::{Query, Configuration};

async fn index(
    schema: web::Data<Schema<Query, EmptyMutation, EmptySubscription>>,
    request: Request,
) -> Response {
    let request = request.into_inner();
    schema.execute(request).await.into()
}

async fn gql_playgound() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
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
    let config = Configuration::from_file(opt.config);
    debug!("{:?}", config);

    print!("Playground: http://localhost:8000/");

    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(config.clone())
        .finish();

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(gql_playgound))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
