use std::pin::Pin;

use crate::log_utils::warn;
use async_openai::{
    config::{AzureConfig, Config, OpenAIConfig},
    error::OpenAIError,
    types::ChatCompletionRequestMessage,
    Client,
};
use entity::entities::{
    conversations::{AzureOptions, ClaudeOptions, DeepseekOptions, GenericOptions, OllamaOptions, OpenAIOptions},
    messages::MessageDTO,
};
use serde::Serialize;
use tokio_stream::{Stream, StreamExt};

use super::{
    providers::{
        claude::{
            chat::{
                ClaudeChat, ClaudeChatCompletionRequest, ClaudeChatCompletionResponseStream,
                ClaudeChatCompletionStreamResponse, ClaudeMessage, ClaudeMetadata,
                ClaudeResponseMessageContent,
            },
            config::ClaudeConfig,
        }, deepseek::{chat::{DeepseekChat, DeepseekChatCompletionRequest, DeepseekChatCompletionResponseStream}, config::DeepseekConfig}, ollama::{
            chat::{
                OllamaChat, OllamaChatCompletionRequest, OllamaChatCompletionResponseStream,
                OllamaMessage,
            },
            config::OllamaConfig,
        }, openai::chat::{OpenAIChat, OpenAIChatCompletionRequest, OpenAIChatCompletionResponseStream, OpenAIChatCompletionStreamOptions}, openrouter::chat::{OpenrouterChat, OpenrouterChatCompletionRequest, OpenrouterChatCompletionResponseStream}
    },
    utils::{message_to_openai_request_message, sum_option},
};

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct BotReply {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(skip_deserializing)]
    pub reasoning: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(skip_deserializing)]
    pub prompt_token: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(skip_deserializing)]
    pub completion_token: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(skip_deserializing)]
    pub reasoning_token: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(skip_deserializing)]
    pub total_token: Option<u32>,
}

pub type BotReplyStream = Pin<Box<dyn Stream<Item = Result<BotReply, OpenAIError>> + Send>>;

pub struct GlobalSettings {
    pub max_tokens: u32,
}

pub enum ChatRequest<'c> {
    OpenAIChatRequest(&'c Client<OpenAIConfig>, OpenAIChatCompletionRequest),
    AzureChatRequest(&'c Client<AzureConfig>, OpenAIChatCompletionRequest),
    ClaudeChatRequest(&'c Client<ClaudeConfig>, ClaudeChatCompletionRequest),
    OllamaChatRequest(&'c Client<OllamaConfig>, OllamaChatCompletionRequest),
    OpenrouterChatRequest(&'c Client<OpenAIConfig>, OpenrouterChatCompletionRequest),
    DeepseekChatRequest(&'c Client<DeepseekConfig>, DeepseekChatCompletionRequest),
}

impl<'c> ChatRequest<'c> {
    pub fn openai(
        client: &'c Client<OpenAIConfig>,
        messages: Vec<MessageDTO>,
        options: GenericOptions,
        global_settings: GlobalSettings,
        model: String,
    ) -> Result<ChatRequest, String> {
        let request: OpenAIChatCompletionRequest;
        // set messages
        let req_messages = messages
            .into_iter()
            .map(message_to_openai_request_message)
            .collect();
        // set options
        let options: OpenAIOptions = serde_json::from_str(&options.options)
            .map_err(|_| format!("Failed to parse conversation options: {}", &options.options))?;
        // build request
        request = OpenAIChatCompletionRequest {
            model: model.to_string(),
            reasoning_effort: options.reasoning_effort.map(|x| x.into()),
            messages: req_messages,
            frequency_penalty: options.frequency_penalty,
            max_tokens: options.max_tokens.or(Some(global_settings.max_tokens)),
            // n: options.n,
            presence_penalty: options.presence_penalty,
            stream: options.stream,
            stream_options: if options.stream.unwrap_or(false) {
                // default to return usage when streaming
                Some(OpenAIChatCompletionStreamOptions {
                    include_usage: true
                })
            } else {
                None
            },
            temperature: options.temperature,
            top_p: options.top_p,
            user: options.user,
            ..Default::default()
        };
        Ok(ChatRequest::OpenAIChatRequest(client, request))
    }

    pub fn azure(
        client: &'c Client<AzureConfig>,
        messages: Vec<MessageDTO>,
        options: GenericOptions,
        global_settings: GlobalSettings,
    ) -> Result<ChatRequest, String> {
        let request: OpenAIChatCompletionRequest;
        // set messages
        let req_messages = messages
            .into_iter()
            .map(message_to_openai_request_message)
            .collect();
        // set options
        let options: AzureOptions = serde_json::from_str(&options.options)
            .map_err(|_| format!("Failed to parse conversation options: {}", &options.options))?;
        // build request
        request = OpenAIChatCompletionRequest {
            messages: req_messages,
            frequency_penalty: options.frequency_penalty,
            max_tokens: options.max_tokens.or(Some(global_settings.max_tokens)),
            // n: options.n,
            presence_penalty: options.presence_penalty,
            stream: options.stream,
            temperature: options.temperature,
            top_p: options.top_p,
            user: options.user,
            ..Default::default()
        };
        Ok(ChatRequest::AzureChatRequest(client, request))
    }

    pub fn claude(
        client: &'c Client<ClaudeConfig>,
        messages: Vec<MessageDTO>,
        options: GenericOptions,
        global_settings: GlobalSettings,
        model: String,
    ) -> Result<ChatRequest, String> {
        let request: ClaudeChatCompletionRequest;
        // set messages
        let req_messages: Vec<ClaudeMessage> = messages
            .into_iter()
            .map(Into::<ClaudeMessage>::into)
            .collect();
        // set options
        let options: ClaudeOptions = serde_json::from_str(&options.options)
            .map_err(|_| format!("Failed to parse conversation options: {}", &options.options))?;
        // build request
        request = ClaudeChatCompletionRequest {
            model: model.to_string(),
            messages: req_messages,
            max_tokens: options.max_tokens.unwrap_or(global_settings.max_tokens),
            stream: options.stream,
            temperature: options.temperature,
            top_p: options.top_p,
            metadata: options.user.map(|user| ClaudeMetadata { user_id: user }),
            ..Default::default()
        };
        Ok(ChatRequest::ClaudeChatRequest(client, request))
    }

    pub fn ollama(
        client: &'c Client<OllamaConfig>,
        messages: Vec<MessageDTO>,
        options: GenericOptions,
        _global_settings: GlobalSettings,
        model: String,
    ) -> Result<ChatRequest, String> {
        let request: OllamaChatCompletionRequest;
        // set messages
        let req_messages: Vec<OllamaMessage> = messages
            .into_iter()
            .map(Into::<OllamaMessage>::into)
            .collect();
        // set options
        let options: OllamaOptions = serde_json::from_str(&options.options)
            .map_err(|_| format!("Failed to parse conversation options: {}", &options.options))?;
        // build request
        // Stream must be set to false explictly for Ollama, or it will treat the request as a Stream request
        let stream = options.stream.clone().unwrap_or(false);
        request = OllamaChatCompletionRequest {
            model: model.to_string(),
            messages: req_messages,
            options: Some(options.into()),
            stream: Some(stream),
            ..Default::default()
        };
        Ok(ChatRequest::OllamaChatRequest(client, request))
    }

    pub fn openrouter(
        client: &'c Client<OpenAIConfig>,
        messages: Vec<MessageDTO>,
        options: GenericOptions,
        global_settings: GlobalSettings,
        model: String,
    ) -> Result<ChatRequest, String> {
        // set messages
        let req_messages: Vec<ChatCompletionRequestMessage> = messages
            .into_iter()
            .map(message_to_openai_request_message)
            .collect();
        // set options
        let options: OpenAIOptions = serde_json::from_str(&options.options)
            .map_err(|_| format!("Failed to parse conversation options: {}", &options.options))?;
        // build request
        let request = OpenrouterChatCompletionRequest {
            model: model.to_string(),
            messages: req_messages,
            frequency_penalty: options.frequency_penalty,
            max_tokens: options.max_tokens.or(Some(global_settings.max_tokens)),
            presence_penalty: options.presence_penalty,
            stream: options.stream,
            temperature: options.temperature,
            top_p: options.top_p,
            include_reasoning: Some(true),
            ..Default::default()
        };
        Ok(ChatRequest::OpenrouterChatRequest(client, request))
    }

    pub fn deepseek(
        client: &'c Client<DeepseekConfig>,
        messages: Vec<MessageDTO>,
        options: GenericOptions,
        global_settings: GlobalSettings,
        model: String,
    ) -> Result<ChatRequest, String> {
        let request: DeepseekChatCompletionRequest;
        // set messages
        let req_messages = messages
            .into_iter()
            .map(message_to_openai_request_message)
            .collect();
        // set options
        let options: DeepseekOptions = serde_json::from_str(&options.options)
            .map_err(|_| format!("Failed to parse conversation options: {}", &options.options))?;
        // build request
        request = DeepseekChatCompletionRequest {
            messages: req_messages,
            model: model.to_string(),
            frequency_penalty: options.frequency_penalty,
            max_tokens: options.max_tokens.or(Some(global_settings.max_tokens)),
            presence_penalty: options.presence_penalty,
            stream: options.stream,
            stream_options: if options.stream.unwrap_or(false) {
                // default to return usage when streaming
                Some(OpenAIChatCompletionStreamOptions {
                    include_usage: true
                })
            } else {
                None
            },
            temperature: options.temperature,
            top_p: options.top_p,
            ..Default::default()
        };
        Ok(ChatRequest::DeepseekChatRequest(client, request))
    }

    async fn execute_openai_compatible_request<C: Config>(
        &self,
        client: &Client<C>,
        request: OpenAIChatCompletionRequest,
    ) -> Result<BotReply, String> {
        let response = OpenAIChat::new(client)
            .create(request)
            .await
            .map_err(|err| {
                log::error!("execute_chat_complete_request: {:?}", err);
                format!("Failed to get chat completion response: {}", err)
            })?;
        // extract data & build reply
        let choice = response
            .choices
            .first()
            .ok_or("Api returned empty choices".to_string())?;
        let message = choice
            .message
            .content
            .as_ref()
            .ok_or("Api returned empty message".to_string())?
            .to_string();
        let usage = response.usage;
        let reply = BotReply {
            message,
            reasoning: None, // OpenAI doesn't return reasoning text yet
            prompt_token: usage.as_ref().map(|usage| usage.prompt_tokens),
            completion_token: usage.as_ref().map(|usage| usage.completion_tokens),
            reasoning_token: usage
                .as_ref()
                .map(|usage| {
                    usage
                        .completion_tokens_details
                        .as_ref()
                        .map(|details| {
                            details.reasoning_tokens.unwrap_or(0)
                        })
                        .unwrap_or(0)
                }),
            total_token: usage.as_ref().map(|usage| usage.total_tokens),
        };

        Ok(reply)
    }

    async fn execute_openai_compatible_stream_request<C: Config>(
        &self,
        client: &Client<C>,
        request: OpenAIChatCompletionRequest,
    ) -> Result<BotReplyStream, String> {
        let stream: OpenAIChatCompletionResponseStream = OpenAIChat::new(client)
            .create_stream(request)
            .await
            .map_err(|err| format!("Error creating stream: {}", err.to_string()))?;
        let result = stream.map(|item| {
            let reply = item.map(|resp| {
                // OpenAI returns usage in the last chunk with an empty message/choice
                let message = resp
                    .choices
                    .first()
                    .map(|choice| {
                        choice.delta.content
                            .clone()
                            .unwrap_or(String::default())
                    })
                    .unwrap_or(String::default());
                let usage = resp.usage;
                BotReply {
                    message,
                    reasoning: None, // OpenAI doesn't return reasoning text yet
                    prompt_token: usage.as_ref().map(|usage| usage.prompt_tokens),
                    completion_token: usage
                        .as_ref()
                        .map(|usage| usage.completion_tokens),
                    reasoning_token: usage
                        .as_ref()
                        .map(|usage| {
                            usage
                                .completion_tokens_details
                                .as_ref()
                                .map(|details| {
                                    details.reasoning_tokens.unwrap_or(0)
                                })
                                .unwrap_or(0)
                        }),
                    total_token: usage.as_ref().map(|usage| usage.total_tokens),
                }
            });
            reply
        });
        Ok(Box::pin(result))
    }

    pub async fn execute(&self) -> Result<BotReply, String> {
        let log_tag = "ChatRequest::execute";
        match self {
            ChatRequest::OpenAIChatRequest(client, request) => {
                return self
                    .execute_openai_compatible_request(client, request.clone())
                    .await;
            }
            ChatRequest::AzureChatRequest(client, request) => {
                return self
                    .execute_openai_compatible_request(client, request.clone())
                    .await;
            }
            ChatRequest::ClaudeChatRequest(client, request) => {
                let response = ClaudeChat::new(client)
                    .create(request.clone())
                    .await
                    .map_err(|err| {
                        log::error!("execute ChatRequest::ClaudeChatRequest: {:?}", err);
                        format!("Failed to get chat completion response: {}", err)
                    })?;
                // extract data & build reply
                let content = response
                    .content
                    .first()
                    .ok_or("Api returned empty content".to_string())?;
                let message = match content {
                    ClaudeResponseMessageContent::Text(text) => text.text.clone(),
                    ClaudeResponseMessageContent::ToolUse(_) => {
                        "ToolUse is not implemented yet".to_string()
                    }
                };
                let usage = response.usage;

                Ok(BotReply {
                    message,
                    reasoning: None,
                    prompt_token: usage.input_tokens,
                    completion_token: usage.output_tokens,
                    reasoning_token: None,
                    total_token: sum_option(usage.input_tokens, usage.output_tokens),
                })
            }
            ChatRequest::OllamaChatRequest(client, request) => {
                let response = OllamaChat::new(client)
                    .create(request.clone())
                    .await
                    .map_err(|err| {
                        log::error!("execute ChatRequest::OllamaChatRequest: {:?}", err);
                        format!("Failed to get chat completion response: {}", err)
                    })?;
                let message: String = match response.message {
                    Some(response_message) => match response_message {
                        OllamaMessage::Assistant(content) => content.content,
                        _ => {
                            warn(
                                log_tag,
                                "OllamaChat::create returned a non-assistant message",
                            );
                            String::default()
                        }
                    },
                    _ => {
                        warn(log_tag, "OllamaChat::create returned an empty message");
                        String::default()
                    }
                };
                // extract data & build reply
                Ok(BotReply {
                    message,
                    reasoning: None,
                    prompt_token: response.prompt_eval_count,
                    completion_token: response.eval_count,
                    reasoning_token: None,
                    total_token: sum_option(response.prompt_eval_count, response.eval_count),
                })
            }
            ChatRequest::OpenrouterChatRequest(client, request) => {
                let response = OpenrouterChat::new(client)
                    .create(request.clone())
                    .await
                    .map_err(|err| {
                        log::error!("execute ChatRequest::OpenrouterChatRequest: {:?}", err);
                        format!("Failed to get chat completion response: {}", err)
                    })?;
                // extract data & build reply
                let choice = response
                    .choices
                    .first()
                    .ok_or("Api returned empty choices".to_string())?;
                let message = choice
                    .message
                    .content
                    .as_ref()
                    .ok_or("Api returned empty message".to_string())?
                    .to_string();
                let usage = response.usage;
                let reply = BotReply {
                    message,
                    reasoning: choice.message.reasoning.clone(),
                    prompt_token: usage.as_ref().map(|usage| usage.prompt_tokens),
                    completion_token: usage.as_ref().map(|usage| usage.completion_tokens),
                    reasoning_token: usage.as_ref().map(|usage| usage.reasoning_tokens.unwrap_or(0)),
                    total_token: usage.as_ref().map(|usage| usage.total_tokens),
                };

                Ok(reply)
            }
            ChatRequest::DeepseekChatRequest(client, request) => {
                let response = DeepseekChat::new(client)
                    .create(request.clone())
                    .await
                    .map_err(|err| format!("Error creating stream: {}", err.to_string()))?;
                // extract data & build reply
                let choice = response
                    .choices
                    .first()
                    .ok_or("Api returned empty choices".to_string())?;
                let message = choice
                    .message
                    .content
                    .as_ref()
                    .ok_or("Api returned empty message".to_string())?
                    .to_string();
                let reasoning = choice
                    .message
                    .reasoning_content
                    .clone();
                let usage = response.usage;
                let reply = BotReply {
                    message,
                    reasoning,
                    prompt_token: usage.as_ref().map(|usage| usage.prompt_tokens),
                    completion_token: usage.as_ref().map(|usage| usage.completion_tokens),
                    reasoning_token: usage
                        .as_ref()
                        .map(|usage| {
                            usage
                                .completion_tokens_details
                                .as_ref()
                                .map(|details| {
                                    details.reasoning_tokens.unwrap_or(0)
                                })
                                .unwrap_or(0)
                        }),
                    total_token: usage.as_ref().map(|usage| usage.total_tokens),
                };

                Ok(reply)
            }
        }
    }

    pub async fn execute_stream(&self) -> Result<BotReplyStream, String> {
        let log_tag = "ChatRequest::execute_stream";
        match self {
            ChatRequest::OpenAIChatRequest(client, request) => {
                return self
                    .execute_openai_compatible_stream_request(client, request.clone())
                    .await;
            }
            ChatRequest::AzureChatRequest(client, request) => {
                return self
                    .execute_openai_compatible_stream_request(client, request.clone())
                    .await;
            }
            ChatRequest::ClaudeChatRequest(client, request) => {
                let stream: ClaudeChatCompletionResponseStream = ClaudeChat::new(client)
                    .create_stream(request.clone())
                    .await
                    .map_err(|err| format!("Error creating stream: {}", err.to_string()))?;
                let result = stream.map(|item| {
                    item.map(|resp| {
                        match resp {
                            ClaudeChatCompletionStreamResponse::ContentBlockDelta(
                                content_delta,
                            ) => BotReply {
                                message: content_delta.delta.text.clone(),
                                ..Default::default()
                            },
                            ClaudeChatCompletionStreamResponse::MessageDelta(message_delta) => {
                                // return empty string as message
                                BotReply {
                                    prompt_token: message_delta.usage.input_tokens,
                                    completion_token: message_delta.usage.output_tokens,
                                    total_token: sum_option(
                                        message_delta.usage.input_tokens,
                                        message_delta.usage.output_tokens,
                                    ),
                                    ..Default::default()
                                }
                            }
                        }
                    })
                });
                Ok(Box::pin(result))
            }
            ChatRequest::OllamaChatRequest(client, request) => {
                let stream: OllamaChatCompletionResponseStream = OllamaChat::new(client)
                    .create_stream(request.clone())
                    .await
                    .map_err(|err| format!("Error creating stream: {}", err.to_string()))?;
                let mut is_reasoning = false;
                let result = stream.map(move |item| {
                    item.map(|response| {
                        let content: String = match response.message {
                            Some(response_message) => match response_message{
                                OllamaMessage::Assistant(content) => {
                                    // check for reasoning content
                                    // return empty content for <think> and </think>
                                    if content.content.contains("<think>") {
                                        is_reasoning = true;
                                        String::default()
                                    } else if content.content.contains("</think>") {
                                        is_reasoning = false;
                                        String::default()
                                    } else {
                                        content.content
                                    }
                                },
                                _ => {
                                    warn(log_tag, "OllamaChat::create_stream returned a non-assistant message");
                                    String::default()
                                }
                            },
                            _ => {
                                // normally the last message of the stream
                                String::default()
                            }
                        };

                        BotReply {
                            message: if is_reasoning {
                                String::default()
                            } else {
                                content.clone()
                            },
                            reasoning: if is_reasoning {
                                Some(content)
                            } else {
                                None
                            },
                            prompt_token: response.prompt_eval_count,
                            completion_token: response.eval_count,
                            reasoning_token: None,
                            total_token: sum_option(response.prompt_eval_count, response.eval_count),
                        }
                    })
                });
                Ok(Box::pin(result))
            }
            ChatRequest::OpenrouterChatRequest(client, request) => {
                let stream: OpenrouterChatCompletionResponseStream = OpenrouterChat::new(&client)
                    .create_stream(request.clone())
                    .await
                    .map_err(|err| format!("Error creating stream: {}", err.to_string()))?;
                let result = stream.map(|item| {
                    item.map(|resp| {
                        let first_choice =
                            resp.choices.first()
                            .map_or(BotReply::default(), |choice| {
                                let usage = resp.usage.clone();
                                let delta = choice.delta.clone();
                                BotReply {
                                    message: delta.content
                                        .clone()
                                        .unwrap_or(String::default()),
                                    reasoning: delta.reasoning
                                        .clone(),
                                    prompt_token: usage.as_ref().map(|usage| usage.prompt_tokens),
                                    completion_token: usage
                                        .as_ref()
                                        .map(|usage| usage.completion_tokens),
                                    reasoning_token: usage
                                        .as_ref()
                                        .map(|usage| usage.reasoning_tokens.unwrap_or(0)),
                                    total_token: usage.as_ref().map(|usage| usage.total_tokens),
                                }
                            });
                        first_choice
                    })
                });
                Ok(Box::pin(result))
            }
            ChatRequest::DeepseekChatRequest(client, request) => {
                let stream: DeepseekChatCompletionResponseStream = DeepseekChat::new(client)
                    .create_stream(request.clone())
                    .await
                    .map_err(|err| format!("Error creating stream: {}", err.to_string()))?;
                let result = stream.map(|item| {
                    let reply = item.map(|resp| {
                        let choice = resp.choices.first().unwrap();
                        let message = choice
                            .delta
                            .content
                            .clone()
                            .unwrap_or(String::default());
                        let reasoning = choice
                            .delta
                            .reasoning_content
                            .clone();
                        let usage = resp.usage;
                        BotReply {
                            message,
                            reasoning,
                            prompt_token: usage.as_ref().map(|usage| usage.prompt_tokens),
                            completion_token: usage.as_ref().map(|usage| usage.completion_tokens),
                            reasoning_token: usage
                                .as_ref()
                                .map(|usage| {
                                    usage
                                        .completion_tokens_details
                                        .as_ref()
                                        .map(|details| {
                                            details.reasoning_tokens.unwrap_or(0)
                                        })
                                        .unwrap_or(0)
                                }),
                            total_token: usage.as_ref().map(|usage| usage.total_tokens),
                            ..Default::default()
                        }
                    });
                    reply
                });
                Ok(Box::pin(result))
            }
        }
    }
}
