//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::{entity::prelude::*, ActiveValue::NotSet, FromQueryResult, Set};
use serde::{Deserialize, Serialize};

pub const DEFAULT_CONTEXT_LENGTH: u16 = 1;
pub const DEFAULT_MAX_TOKENS: u16 = 256;

#[derive(Clone, Default, Debug, PartialEq, DeriveEntityModel, Eq, Deserialize, Serialize)]
#[sea_orm(table_name = "conversations")]
#[serde(rename_all = "camelCase")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<i32>,
    pub subject: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(skip_deserializing)]
    pub options: Option<String>,
    #[serde(skip_deserializing)]
    pub created_at: DateTimeLocal,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(skip_deserializing)]
    pub updated_at: Option<DateTimeLocal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(skip_deserializing)]
    pub deleted_at: Option<DateTimeLocal>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::messages::Entity")]
    Messages,
    #[sea_orm(
        belongs_to = "super::models::Entity",
        from = "Column::ModelId",
        to = "super::models::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Models,
}

impl Related<super::messages::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Messages.def()
    }
}

impl Related<super::models::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Models.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

pub type ConversationDTO = Model;
#[derive(Clone, Debug, FromQueryResult, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationDetailsDTO {
    pub id: i32,
    pub model_id: Option<i32>,
    pub subject: String,
    pub options: Option<String>,
    pub created_at: DateTimeLocal,
    pub updated_at: Option<DateTimeLocal>,
    pub message_count: Option<i32>,
    pub model_provider: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewConversationDTO {
    pub model_id: i32,
    pub message: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateConversationDTO {
    pub id: i32,
    pub model_id: Option<i32>,
    pub subject: Option<String>,
    pub options: Option<String>,
}

impl From<UpdateConversationDTO> for ActiveModel {
    fn from(value: UpdateConversationDTO) -> Self {
        Self {
            id: Set(value.id),
            model_id: value.model_id.map_or(NotSet, |x| Set(Some(x))),
            subject: value.subject.map_or(NotSet, |x| Set(x)),
            options: value.options.map_or(NotSet, |x| Set(Some(x))),
            created_at: NotSet,
            updated_at: NotSet,
            deleted_at: NotSet,
        }
    }
}

#[derive(Clone, Debug, FromQueryResult, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProviderOptions {
    pub provider: String,
    pub options: String
}

pub trait Options {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AzureOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_length: Option<u16>,
    // pub best_of: Option<i32>, // async-openai currently doesn't support this
    // pub echo: Option<bool>, // async-openai currently doesn't support this
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>, // min: -2.0, max: 2.0, default: 0
    // pub function_call: Option<ChatCompletionFunctionCall>,
    // pub functions: Option<Vec<ChatCompletionFunctions>>,
    // pub logit_bias: Option<HashMap<String, serde_json::Value>>, // default: null
    // pub logprobs: Option<i32>, // Azure seems to have a different definition from OpenAI's. async-openai currently doesn't support the Azure version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u8>, // min:1, max: 128, default: 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>, // min: -2.0, max: 2.0, default 0
    // pub response_format: Option<ChatCompletionResponseFormat>, // to be implemented
    // pub seed: Option<i64>, // not supported by Azure
    // pub stop: Option<Stop>, // to be implemented
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    // pub suffix: Option<String>, // async-openai currently doesn't support this
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>, // min: 0, max: 2, default: 1,
    // pub tools: Option<Vec<ChatCompletionTool>>,
    // pub tool_choice: Option<ChatCompletionToolChoiceOption>,
    // pub top_logprobs: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>, // min: 0, max: 1, default: 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl Options for AzureOptions {}

impl Default for AzureOptions{
    fn default() -> Self {
        AzureOptions {
            context_length: None,
            frequency_penalty: Some(0.0),
            max_tokens: None,
            n: Some(1),
            presence_penalty: Some(0.0),
            stream: Some(false),
            temperature: Some(1.0),
            top_p: Some(1.0),
            user: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenAIOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_length: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>, // min: -2.0, max: 2.0, default: 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u8>, // min:1, max: 128, default: 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>, // min: -2.0, max: 2.0, default 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>, // min: 0, max: 2, default: 1,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>, // min: 0, max: 1, default: 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl Options for OpenAIOptions {}

impl Default for OpenAIOptions{
    fn default() -> Self {
        OpenAIOptions {
            context_length: None,
            frequency_penalty: Some(0.0),
            max_tokens: None,
            n: Some(1),
            presence_penalty: Some(0.0),
            stream: Some(false),
            temperature: Some(1.0),
            top_p: Some(1.0),
            user: None,
        }
    }
}