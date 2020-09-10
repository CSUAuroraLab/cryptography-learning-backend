use async_graphql::{Object, SimpleObject};
pub struct Query;

#[Object]
impl Query {
    async fn hello(&self) -> String {
        "hello world".to_string()
    }

    async fn practice(&self) -> Practice {
        Practice {
            lab_categories: LabCategory {
                labs: vec![Lab {
                    name: String::from("test"),
                    resource: String::from("/test"),
                    endpoints: vec![Endpoint {
                        host: String::from("http://localhost"),
                        port: 114514,
                    }],
                }],
            },
        }
    }
}

#[SimpleObject]
struct Endpoint {
    host: String,
    port: i32,
}

#[SimpleObject]
struct Lab {
    name: String,
    resource: String,
    endpoints: Vec<Endpoint>,
}

#[SimpleObject]
struct LabCategory {
    labs: Vec<Lab>,
}

#[SimpleObject]
struct Practice {
    lab_categories: LabCategory,
}
