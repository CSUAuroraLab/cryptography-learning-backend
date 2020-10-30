use async_graphql::{Object, SimpleObject};
use serde::Deserialize;
pub struct Query;

#[Object]
impl Query {
    async fn hello(&self) -> String {
        "hello world".to_string()
    }

    async fn practice(&self) -> Practice {
        Practice {
            lab_categories: vec![LabCategory {
                name: String::from("test"),
                labs: vec![Lab {
                    name: String::from("test"),
                    resource: String::from("/test"),
                    endpoints: vec![Endpoint {
                        host: String::from("http://localhost"),
                        port: 114514,
                    }],
                }],
            }],
        }
    }
}

#[derive(Deserialize, SimpleObject)]
struct Endpoint {
    host: String,
    port: i32,
}

#[derive(Deserialize, SimpleObject)]
struct Lab {
    name: String,
    resource: String,
    endpoints: Vec<Endpoint>,
}

#[derive(Deserialize, SimpleObject)]
struct LabCategory {
    name: String,
    labs: Vec<Lab>,
}

#[derive(Deserialize, SimpleObject)]
struct Practice {
    lab_categories: Vec<LabCategory>,
}

#[derive(Deserialize, SimpleObject)]
struct Translation {
    lang: String,
    text: String,
}