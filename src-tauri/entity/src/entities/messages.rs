//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;
use sea_orm::entity::Linked;
use sea_orm::ActiveValue;
use sea_orm::FromJsonQueryResult;
use sea_orm::IntoActiveModel;
use sea_orm::IntoActiveValue;
use sea_orm::Set;
use serde::{Deserialize, Serialize};

use super::contents::ContentType;

pub enum Roles {
    User,
    Bot,
    System,
}

impl Into<i32> for Roles {
    fn into(self) -> i32 {
        match self {
            Roles::User => 0,
            Roles::Bot => 1,
            Roles::System => 2,
        }
    }
}

impl From<i32> for Roles {
    fn from(value: i32) -> Self {
        match value {
            0 => Roles::User,
            1 => Roles::Bot,
            2 => Roles::System,
            _ => panic!("Invalid role"),
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "messages")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub conversation_id: i32,
    pub role: i32,
    // pub content: ContentItemList,
    #[serde(skip_deserializing)]
    pub created_at: DateTimeLocal,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(skip_deserializing)]
    pub updated_at: Option<DateTimeLocal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(skip_deserializing)]
    pub deleted_at: Option<DateTimeLocal>,
}

#[derive(Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
pub struct ContentItemList {
    pub items: Vec<ContentItem>
}

impl ContentItemList {
    pub fn new(item: ContentItem) -> Self {
        ContentItemList {
            items: vec![item]
        }
    }
}

impl IntoActiveValue<String> for ContentItemList {
    fn into_active_value(self) -> ActiveValue<String> {
        ActiveValue::Set(serde_json::to_string(&self).unwrap_or(String::default()))
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::conversations::Entity",
        from = "Column::ConversationId",
        to = "super::conversations::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Conversations,
    #[sea_orm(has_many = "super::contents::Entity")]
    Contents,
}

impl Related<super::conversations::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Conversations.def()
    }
}

impl Related<super::contents::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Contents.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

/**
 * Relation link from Message to Model
 */
pub struct MessageToModel;

impl Linked for MessageToModel {
    type FromEntity = Entity;
    type ToEntity = super::models::Entity;

    fn link(&self) -> Vec<RelationDef> {
        vec![
            Relation::Conversations.def(),
            super::conversations::Relation::Models.def(),
        ]
    }
}

// #[derive(Clone, Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct NewMessage {
//     pub conversation_id: i32,
//     pub role: i32,
//     pub content: ContentItemList,
// }

// impl IntoActiveModel<ActiveModel> for NewMessage {
//     fn into_active_model(self) -> ActiveModel {
//         ActiveModel {
//             conversation_id: ActiveValue::Set(self.conversation_id),
//             role: ActiveValue::Set(self.role),
//             // content: ActiveValue::Set(self.content),
//             ..Default::default()
//         }
//     }
// }

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromJsonQueryResult)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ContentItem {
    Text { data: String },
    Image { file_name: String, file_size: u32, file_type: String, file_last_modified: u32, file_data: Vec<u8> }
}

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageDTO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub conversation_id: i32,
    pub role: i32,
    // pub content: ContentItemList,
    #[serde(skip_deserializing)]
    pub created_at: DateTimeLocal,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(skip_deserializing)]
    pub updated_at: Option<DateTimeLocal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(skip_deserializing)]
    pub deleted_at: Option<DateTimeLocal>,
    pub content: Vec<ContentItem>,
}

impl MessageDTO {
    pub fn get_text(&self) -> Option<String> {
        self.content
            .iter()
            .find_map(|item| {
                if let ContentItem::Text{data} = item {
                    Some(data.clone())
                } else {
                    None
                }
            })
    }
}

impl From<(Model, Vec<super::contents::Model>)> for MessageDTO {
    fn from(value: (Model, Vec<super::contents::Model>)) -> Self {
        let message = value.0;
        let contents = value.1;
        MessageDTO {
            id: Some(message.id),
            conversation_id: message.conversation_id,
            role: message.role,
            created_at: message.created_at,
            updated_at: message.updated_at,
            deleted_at: message.deleted_at,
            content: contents.into_iter().map(|content| content.into()).collect(),
        }
    }
}

impl IntoActiveModel<ActiveModel> for MessageDTO {
    fn into_active_model(self) -> ActiveModel {
        ActiveModel {
            conversation_id: ActiveValue::Set(self.conversation_id),
            role: ActiveValue::Set(self.role),
            // content: ActiveValue::Set(self.content),
            ..Default::default()
        }
    }
}

impl From<super::contents::Model> for ContentItem {
    fn from(value: super::contents::Model) -> Self {
        match value.r#type {
            ContentType::Text => {
                ContentItem::Text {
                    data: value.text.unwrap_or_default()
                }
            },
            ContentType::Image => {
                ContentItem::Image {
                    file_name: value.file_name.unwrap_or_default(),
                    file_size: value.file_size.unwrap_or_default(),
                    file_type: value.file_type.unwrap_or_default(),
                    file_last_modified: value.file_last_modified.unwrap_or_default(),
                    file_data: value.file_data.unwrap_or_default(),
                }
            },
        }
    }
}

impl IntoActiveModel<super::contents::ActiveModel> for ContentItem {
    fn into_active_model(self) -> super::contents::ActiveModel {
        match self {
            ContentItem::Text { data } => {
                super::contents::ActiveModel {
                    r#type: Set(ContentType::Text),
                    text: Set(Some(data)),
                    ..Default::default()
                }
            },
            ContentItem::Image { file_name, file_size, file_type, file_last_modified, file_data } => {
                super::contents::ActiveModel {
                    r#type: Set(ContentType::Image),
                    file_name: Set(Some(file_name)),
                    file_size: Set(Some(file_size)),
                    file_type: Set(Some(file_type)),
                    file_last_modified: Set(Some(file_last_modified)),
                    ..Default::default()
                }
            },
        }
    }
}