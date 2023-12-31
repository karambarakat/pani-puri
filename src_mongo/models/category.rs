use std::borrow::Cow;

use async_graphql::*;
use bson::Bson;
// use futures_util::StreamExt;

use mongodb::bson::doc;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, Default)]
struct MyID(pub String);

// implement display
impl std::fmt::Display for MyID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

// impl From<async_graphql::ID> for MyID {
//     fn from(id: ID) -> Self {
//         MyID(id.to_string())
//     }
// }

// impl From<mongodb::bson::oid::ObjectId> for MyID {
//     fn from(id: mongodb::bson::oid::ObjectId) -> Self {
//         MyID(id.to_hex())
//     }
// }

#[async_trait::async_trait]
impl OutputType for MyID {
    fn type_name() -> Cow<'static, str> {
        <ID as async_graphql::OutputType>::type_name()
    }

    fn create_type_info(registry: &mut async_graphql::registry::Registry) -> String {
        <ID as async_graphql::OutputType>::create_type_info(registry)
    }

    async fn resolve(
        &self,
        ctx: &ContextSelectionSet<'_>,
        field: &Positioned<async_graphql::parser::types::Field>,
    ) -> ServerResult<Value> {
        async_graphql::ID::resolve(&self.into(), ctx, field).await
    }
}

// #[derive(Default, SimpleObject, Serialize, Deserialize)]
// struct EntryTest {
//     #[serde(with = "crate::oidser")]
//     pub _id: ID,
//     pub color: Option<String>,
//     pub icon: Option<String>,
//     pub title: String,
// }

#[derive(Default, SimpleObject)]
pub struct Category {
    pub _id: ID,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub title: String,
}

impl From<bson::Document> for Category {
    fn from(value: bson::Document) -> Self {
        Category {
            _id: match value.get("_id").expect("invalid data: _id missing") {
                Bson::ObjectId(t) => ID(t.to_hex()),
                _ => panic!("invalid data: _id is not objectID"),
            },
            color: match value.get("color") {
                Some(Bson::String(t)) => Some(t.to_string()),
                None => None,
                _ => panic!("invalid data: color is not string"),
            },
            icon: match value.get("icon") {
                Some(Bson::String(t)) => Some(t.to_string()),
                None => None,
                _ => panic!("invalid data: icon is not string"),
            },
            title: match value.get("title").expect("invalid data: title missing") {
                Bson::String(t) => t.to_string(),
                _ => panic!("invalid data: title is not string"),
            },
        }
    }
}

#[derive(Default, Deserialize, Serialize)]
pub struct CategoryDB {
    pub _id: mongodb::bson::oid::ObjectId,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub title: String,
}

// from EntryDB to Entry
impl Into<Category> for CategoryDB {
    fn into(self) -> Category {
        Category {
            _id: self._id.into(),
            color: self.color,
            icon: self.icon,
            title: self.title,
        }
    }
}

#[derive(InputObject, Clone, Deserialize, Serialize)]
pub struct CategoryInput {
    pub color: Option<String>,
    pub icon: Option<String>,
    pub title: String,
}

// convert from category input to category
impl CategoryInput {
    pub fn inserted(self, inserted_id: Bson) -> Category {
        Category {
            _id: ID(inserted_id.as_object_id().unwrap().to_hex()).into(),
            color: self.color,
            icon: self.icon,
            title: self.title,
        }
    }
}

#[derive(InputObject, Clone, Deserialize, Serialize)]
pub struct CategoryUpdate {
    pub _id: ID,
    pub color: Option<String>,
    pub icon: Option<String>,
    pub title: Option<String>,
}

impl CategoryUpdate {
    pub fn into_doc(self) -> bson::Document {
        let mut doc = doc! {};

        if let Some(color) = &self.color {
            doc.insert("color", color);
        }

        if let Some(icon) = &self.icon {
            doc.insert("icon", icon);
        }

        if let Some(title) = &self.title {
            doc.insert("title", title);
        }

        doc
    }
}
