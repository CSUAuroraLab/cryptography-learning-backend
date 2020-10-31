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
                name: vec![Translation{ 
                    lang: String::from("en"),
                    text: String::from("test"),
                }],
                labs: vec![Lab {
                    name: vec![Translation{ 
                        lang: String::from("en"),
                        text: String::from("test"),
                    }],
                    resource: String::from("/test"),
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
    name: Vec<Translation>,
    #[graphql(skip)]
    resource: String,
}

#[derive(Deserialize, SimpleObject)]
struct LabCategory {
    name: Vec<Translation>,
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