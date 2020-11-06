use crate::errors::QueryError;
use async_graphql::{Context, ErrorExtensions, FieldResult, Object, SimpleObject};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read, path::PathBuf};

pub struct Query;

#[Object]
impl Query {
    async fn hello(&self) -> String {
        "hello world".to_string()
    }

    async fn practice<'ctx>(&self, ctx: &'ctx Context<'_>) -> FieldResult<&'ctx Practice> {
        match ctx.data::<Configuration>() {
            Ok(configuration) => Ok(&configuration.practice),
            Err(e) => Err(e),
        }
    }

    async fn lab<'ctx>(
        &self,
        ctx: &'ctx Context<'_>,
        category_id: String,
        lab_id: String,
        language: Option<String>,
    ) -> FieldResult<LabInstance> {
        let categories = &ctx.data::<Configuration>()?.practice.lab_categories;

        let labs = &categories
            .iter()
            .find(|category| category.id == category_id)
            .ok_or(QueryError::NotFoundError("category".to_string()).extend())?
            .labs;

        let lab = labs
            .iter()
            .find(|lab| lab.id == lab_id)
            .ok_or(QueryError::NotFoundError("lab".to_string()).extend())?;

        let resource = language
            .map(|lang| lab.resources.iter().find(|resource| resource.lang == lang))
            .flatten()
            .ok_or(QueryError::NotFoundError("translation".to_string()).extend())?;

        let mut result: LabInstance = LabInstance {
            lang: resource.lang.clone(),
            name: resource.name.clone(),
            content: String::new(),
        };
        File::open(&resource.resource)
            .map_err(|_| QueryError::ServerError("internal error".to_string()).extend())?
            .read_to_string(&mut result.content)
            .map_err(|_| QueryError::ServerError("internal error".to_string()).extend())?;

        Ok(result)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Configuration {
    practice: Practice,
}

impl Configuration {
    pub fn from_file(path: PathBuf) -> Configuration {
        let mut config_file = File::open(path).expect("Error occurred opening file");
        let mut config_string = String::new();
        config_file
            .read_to_string(&mut config_string)
            .expect("Error Reading file");
        ron::from_str(&config_string).expect("Error parsing file")
    }
}

#[derive(Deserialize, Serialize, SimpleObject, Debug, Clone)]
struct Endpoint {
    host: String,
    port: i32,
}

#[derive(Deserialize, Serialize, SimpleObject, Debug, Clone)]
struct Lab {
    id: String,
    resources: Vec<ResourceWithTranslation>,
}

#[derive(Deserialize, Serialize, SimpleObject, Debug, Clone)]
struct LabInstance {
    lang: String,
    name: String,
    content: String,
}

#[derive(Deserialize, Serialize, SimpleObject, Debug, Clone)]
struct LabCategory {
    id: String,
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

#[derive(Deserialize, Serialize, SimpleObject, Debug, Clone)]
struct ResourceWithTranslation {
    #[serde(rename(deserialize = "language", serialize = "language"))]
    lang: String,
    name: String,
    #[graphql(skip)]
    resource: String,
}
