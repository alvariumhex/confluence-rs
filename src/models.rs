use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Space {
    pub id: u64,
    pub key: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PageChildren {
    pub page: PaginatedResponse<Page>,
    #[serde(rename = "_links")]
    pub links: Links,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Page {
    pub id: String,
    pub title: String,
    pub status: String,
    pub space: Option<Space>,
    pub body: Option<Body>,
    #[serde(rename = "_links")]
    pub links: Links,
    pub children: Option<PageChildren>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Body {
    pub view: Option<BodyView>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BodyView {
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SpacesResult {
    pub results: Vec<Space>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Links {
    pub next: Option<String>,
    pub prev: Option<String>,
    pub base: Option<String>,
    pub webui: Option<String>,
    #[serde(rename = "self")]
    pub _self: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PaginatedResponse<T> {
    pub size: u64,
    pub limit: u64,
    pub start: u64,
    #[serde(rename = "_links")]
    pub links: Links,
    pub results: Vec<T>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SpaceContentResult {
    pub page: PaginatedResponse<Page>,
}
