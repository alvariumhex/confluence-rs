use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Space {
    pub id: u64,
    pub key: String,
    pub name: String,
    #[serde(rename = "_expandable")]
    pub expandable: SpaceExpandable,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SpaceExpandable {
    pub homepage: Option<String>,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostPage {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub t: String,
    pub title: String,
    pub ancestors: Option<Vec<PostAncestor>>,
    pub space: PostSpace,
    pub body: Option<PostBody>,
    pub version: Option<PostVersion>,
}
impl PostPage {
    pub fn new_new_page(
        title: String,
        ancestor: Option<u64>,
        space_key: String,
        body: Option<String>,
    ) -> Self {
        PostPage {
            id: None,
            t: "page".to_string(),
            title,
            ancestors: ancestor.map(|ancestor_id| {
                vec![PostAncestor {
                    id: format!("{ancestor_id}"),
                }]
            }),
            space: PostSpace { key: space_key },
            body: body.map(PostBody::new),
            version: None,
        }
    }
    pub fn new_update_page(
        id: u64,
        title: String,
        space_key: String,
        body: Option<String>,
        new_version: u64,
    ) -> Self {
        PostPage {
            id: Some(format!("{id}")),
            t: "page".to_string(),
            title,
            ancestors: None,
            space: PostSpace { key: space_key },
            body: body.map(PostBody::new),
            version: Some(PostVersion {
                number: new_version,
            }),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostAncestor {
    pub id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostSpace {
    pub key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostBody {
    pub storage: PostStorage,
}
impl PostBody {
    pub fn new(value: String) -> Self {
        PostBody {
            storage: PostStorage {
                value,
                representation: "storage".to_string(),
            },
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostStorage {
    pub value: String,
    pub representation: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostVersion {
    pub number: u64,
}
