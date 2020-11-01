use async_graphql::{Context, Object, SimpleObject};
use serde::{Deserialize, Serialize};
use std::{fs::File, path::PathBuf, io::Read};


pub struct Query;

#[Object]
impl Query {
    async fn hello(&self) -> String {
        "hello world".to_string()
    }

    async fn practice<'ctx>(&self, ctx: &'ctx Context<'_>) -> Result<&'ctx Practice, async_graphql::Error> {
        match ctx.data::<Configuration>() {
            Ok(configuration) => Ok(&configuration.practice),
            Err(e) => Err(e)
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Configuration {
    practice: Practice
}

impl Configuration {
    pub fn from_file(path: PathBuf) -> Configuration {
        let mut config_file = match File::open(path) {
            Ok(f) => f,
            Err(e) => panic!("Error occurred opening file: {}", e)
        };
        let mut config_string = String::new();
        match config_file.read_to_string(&mut config_string) {
            Ok(s) => s,
            Err(e) => panic!("Error Reading file: {}", e)
        };
        match toml::from_str(&config_string) {
            Ok(config) => config,
            Err(e) => panic!("Error parsing file: {}", e)
        }
    }
}

#[derive(Deserialize, Serialize, SimpleObject, Debug, Clone)]
struct Endpoint {
    host: String,
    port: i32,
}

#[derive(Deserialize, Serialize, SimpleObject, Debug, Clone)]
struct Lab {
    #[graphql(skip)]
    resource: String,
    name: Vec<Translation>,
}

#[derive(Deserialize, Serialize, SimpleObject, Debug, Clone)]
struct LabCategory {
    id: i32,
    name: Vec<Translation>,
    labs: Vec<Lab>,
}

#[derive(Deserialize, Serialize, SimpleObject, Debug, Clone)]
struct Practice {
    lab_categories: Vec<LabCategory>,
}

#[derive(Deserialize, Serialize, SimpleObject, Debug, Clone)]
struct Translation {
    #[serde(rename(deserialize = "language", serialize = "language"))]
    lang: String,
    text: String,
}