#[macro_use]
extern crate thiserror;

use actix_web::{guard, web, App, HttpResponse, HttpServer};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{
    EmptyMutation, EmptySubscription, ErrorExtensions, FieldError, FieldResult, Object, ResultExt,
    Schema,
};
use async_graphql_actix_web::{GQLRequest, GQLResponse};
use serde_json::json;

struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn hello(&self) -> String {
        "hello world".to_string()
    }
}

async fn index(
    schema: web::Data<Schema<QueryRoot, EmptyMutation, EmptySubscription>>,
    req: GQLRequest,
) -> GQLResponse {
    req.into_inner().execute(&schema).await.into()
}

async fn gql_playgound() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Playground: http://localhost:8000/");

    HttpServer::new(move || {
        App::new()
            .data(Schema::new(QueryRoot, EmptyMutation, EmptySubscription))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(gql_playgound))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
