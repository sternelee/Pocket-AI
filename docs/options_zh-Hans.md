# 🤖支持的配置项

[![en-icon]](./options.md)
[![zh-hans-icon]](./options_zh-Hans.md)

**符号:** ✅ - 支持, ❌ - 不支持, 📌 - 计划支持

## OpenAI ✅

### API 配置

| 字段 | 描述 |
| -------- | -------- |
| API Key | 您的 OpenAI API 密钥。 |
| Model | 要使用的模型 ID。 |

### 对话选项

| 选项 | 描述 | 支持 |
| - | - | - |
| frequency_penalty | 介于 -2.0 和 2.0 之间的数字。正值会根据文本中已有的频率对新词元进行惩罚，降低模型逐字重复相同内容的可能性。 | ✅ |
| max_tokens | 聊天完成中可以生成的最大词元数。<br/>输入词元和生成词元的总长度受模型上下文长度的限制。 | ✅ |
| presence_penalty | 介于 -2.0 和 2.0 之间的数字。正值会根据新词元是否已出现在文本中进行惩罚，增加模型谈论新主题的可能性。 | ✅ |
| temperature | 使用的采样温度，介于 0 和 2 之间。较高的值（如 0.8）会使输出更随机，而较低的值（如 0.2）会使其更加集中和确定性。<br/>我们通常建议调整此参数或 top_p，但不要同时调整两者。 | ✅ |
| top_p | 作为温度采样的替代方法，称为核采样，模型考虑具有 top_p 概率质量的词元结果。因此，0.1 意味着只考虑构成前 10% 概率质量的词元。<br/>我们通常建议调整此参数或 temperature，但不要同时调整两者。 | ✅ |
| stream | 如果设置，将发送部分消息增量，类似于 ChatGPT。 | ✅ |
| user | 代表您的终端用户的唯一标识符，可以帮助 OpenAI 监控和检测滥用行为。 | ✅ |
| response_format | 指定模型必须输出的格式的对象。兼容 GPT-4 Turbo 和所有比 gpt-3.5-turbo-1106 更新的 GPT-3.5 Turbo 模型。 | 📌 |
| seed | 如果指定，我们的系统将尽最大努力进行确定性采样，使得具有相同种子和参数的重复请求应返回相同的结果。 | 📌 |
| stop | 最多 4 个序列，API 将在这些序列处停止生成更多词元。 | 📌 |
| tools | 模型可能调用的工具列表。目前，仅支持函数作为工具。使用此选项提供模型可能生成 JSON 输入的函数列表。 | ❌ |
| tool_choice | 控制模型是否（以及如何）调用函数。none 表示模型不会调用函数，而是生成一条消息。auto 表示模型可以在生成消息或调用函数之间选择。通过指定特定函数（如 {"type": "function", "function": {"name": "my_function"}}）强制模型调用该函数。<br/>当没有函数存在时，默认为 none。如果存在函数，默认为 auto。 | ❌ |
| logit_bias | 修改指定词元在完成中出现的可能性。<br/>接受一个 JSON 对象，该对象将词元（由分词器中的词元 ID 指定）映射到 -100 到 100 之间的相关偏差值。从数学上讲，在采样之前，偏差被添加到模型生成的对数概率中。确切效果因模型而异，但 -1 到 1 之间的值应该降低或增加选择的可能性；-100 或 100 这样的值应该导致相关词元被禁用或独占选择。 | ❌ |
| logprobs | 是否返回输出词元的对数概率。如果为 true，则在消息内容中返回每个输出词元的对数概率。此选项目前在 gpt-4-vision-preview 模型上不可用。 | ❌ |
| top_logprobs | 一个介于 0 和 5 之间的整数，指定在每个词元位置返回的最可能词元数量，每个词元都有相关的对数概率。如果使用此参数，必须将 logprobs 设置为 true。 | ❌ |
| n | 为每个输入消息生成的聊天完成选项数量。请注意，您将根据所有选项中生成的词元数量进行收费。将 n 保持为 1 以最小化成本。 | ❌ |

### 参考资料

- [OpenAI Documentation](https://platform.openai.com/docs/guides/text-generation/chat-completions-api)

## Microsoft Azure ✅

### API 配置

| 字段 | 描述 |
| -------- | -------- |
| API 密钥 | 您的 Azure OpenAI API 的 API 密钥。 |
| 端点 | 您的 Azure OpenAI API 的端点。 |
| API 版本 | 用于此操作的 API 版本。遵循 YYYY-MM-DD 或 YYYY-MM-DD-preview 格式。 |
| 部署 ID | 您的模型部署名称。 |

### 对话选项

| 选项 | 描述 | 支持 |
| - | - | - |
| max_tokens | 在补全中生成的最大令牌数。您的提示的令牌数加上 max_tokens 不能超过模型的上下文长度。 | ✅ |
| temperature | 使用的采样温度，介于 0 和 2 之间。较高的值意味着模型承担更多风险。对于更具创造性的应用，尝试使用 0.9，对于有明确答案的应用，使用 0（最大概率采样）。我们通常建议调整此参数或 top_p，但不要同时调整两者。 | ✅ |
| top_p | 温度采样的替代方法，称为核采样，模型考虑具有 top_p 概率质量的令牌结果。因此，0.1 意味着只考虑构成前 10% 概率质量的令牌。我们通常建议调整此参数或 temperature，但不要同时调整两者。 | ✅ |
| presence_penalty | 介于 -2.0 和 2.0 之间的数字。正值根据新令牌是否出现在目前的文本中进行惩罚，增加模型谈论新主题的可能性。 | ✅ |
| frequency_penalty | 介于 -2.0 和 2.0 之间的数字。正值根据新令牌在目前文本中的现有频率进行惩罚，降低模型逐字重复相同内容的可能性。 | ✅ |
| stream | 如果设置，将发送部分消息增量，类似于 ChatGPT。 | ✅ |
| user | 代表您的最终用户的唯一标识符，可以帮助 OpenAI 监控和检测滥用行为。 | ✅ |
| suffix | 插入文本补全后的后缀。 | 📌 |
| echo | 除了补全之外，还回显提示。此参数不能与 gpt-35-turbo 一起使用。 | 📌 |
| stop | 最多四个序列，API 将在这些序列处停止生成更多令牌。返回的文本不会包含停止序列。对于 GPT-4 Turbo with Vision，最多支持两个序列。 | 📌 |
| logit_bias | 修改指定令牌在补全中出现的可能性。接受一个 json 对象，该对象将令牌（由 GPT 分词器中的令牌 ID 指定）映射到 -100 到 100 之间的相关偏差值。您可以使用此分词器工具（适用于 GPT-2 和 GPT-3）将文本转换为令牌 ID。从数学上讲，偏差在采样之前被添加到模型生成的对数概率中。确切效果因模型而异，但 -1 到 1 之间的值应该会降低或增加选择的可能性；像 -100 或 100 这样的值应该会导致相关令牌被禁止或独家选择。例如，您可以传递 {"50256": -100} 以防止生成 <\|endoftext\|> 令牌。 | ❌ |
| n | 为每个输入消息生成多少个聊天补全选项。请注意，您将根据所有选项中生成的令牌数量被收费。将 n 保持为 1 以最小化成本。 | ❌ |
| logprobs | 在 logprobs 最可能的令牌上包含对数概率，以及所选令牌。例如，如果 logprobs 为 10，API 将返回 10 个最可能的令牌列表。API 将始终返回采样令牌的对数概率，因此响应中可能最多有 logprobs+1 个元素。此参数不能与 gpt-35-turbo 一起使用。 | ❌ |
| best_of | 在服务器端生成 best_of 补全并返回"最佳"（每个令牌对数概率最低的那个）。结果不能流式传输。当与 n 一起使用时，best_of 控制候选补全的数量，n 指定要返回的数量 – best_of 必须大于 n。注意：因为此参数生成许多补全，它可能会快速消耗您的令牌配额。请谨慎使用，并确保您对 max_tokens 和 stop 有合理的设置。此参数不能与 gpt-35-turbo 一起使用。 | ❌ |

### 参考资料

- [Azure Documentation](https://learn.microsoft.com/en-us/azure/ai-services/openai/reference#chat-completions)

以下是翻译后的技术文档,保留了原有的markdown格式:

## Anthropic Claude ✅

### API 配置

| 字段 | 描述 |
| -------- | -------- |
| api-key | 您的 Anthropic API 密钥。 |
| anthropic-version | 要使用的 Anthropic 版本。 |
| model | 要使用的 Anthropic 模型。 |

### 对话选项

| 选项 | 描述 | 支持情况 |
| - | - | - |
| max_tokens | 生成停止前的最大令牌数。 | ✅ |
| temperature | 注入响应中的随机性程度。<br/>默认为 1.0。范围从 0.0 到 1.0。对于分析/多选题使用接近 0.0 的温度,对于创意和生成任务使用接近 1.0 的温度。<br/>我们通常建议调整此项或 top_p,但不要同时调整两者。 | ✅ |
| top_p | 使用核采样。<br/>仅推荐高级用例使用。通常您只需要使用 _temperature_。| ✅ |
| stream | 是否使用服务器发送事件增量流式传输响应。 | ✅ |
| user | 描述请求元数据的对象。<br/>_metadata.user_id_: 与请求关联的用户的外部标识符。 | ✅ |
| stop_sequences | 导致模型停止生成的自定义文本序列。 | 📌 |
| top_k | 仅从每个后续令牌的前 K 个选项中采样。<br/>仅推荐高级用例使用。通常您只需要使用 _temperature_。| 📌 |
| tools | 模型可能使用的工具定义。 | ❌ |
| tool_choice | 模型应如何使用提供的工具。 | ❌ |

### 参考资料

- [Anthropic API](https://docs.anthropic.com/en/api/messages)

## Ollama ✅

### API 配置

| 字段 | 描述 |
| - | - |
| Endpoint | Azure OpenAI API 的端点。 |
| Model | 要使用的模型。 |

### 对话选项

| 选项 | 描述 | 支持 |
| - | - | - |
| num_ctx | 输入令牌数。设置用于生成下一个令牌的上下文窗口大小。（默认值：2048） | ✅ |
| num-predict | 输出令牌数。生成文本时预测的最大令牌数。（默认值：128，-1 = 无限生成，-2 = 填充上下文） | ✅ |
| temperature | 模型的温度。增加温度将使模型回答更具创造性。（默认值：0.8） | ✅ |
| top_p | 与 top-k 一起工作。较高的值（如 0.95）将产生更多样化的文本，而较低的值（如 0.5）将生成更集中和保守的文本。（默认值：0.9） | ✅ |
| mirostat | 启用 Mirostat 采样以控制困惑度。（默认值：0，0 = 禁用，1 = Mirostat，2 = Mirostat 2.0） | 📌 |
| mirostat_eta | 影响算法对生成文本反馈的响应速度。较低的学习率将导致调整较慢，而较高的学习率将使算法更加敏感。（默认值：0.1） | 📌 |
| mirostat_tau | 控制输出的连贯性和多样性之间的平衡。较低的值将产生更集中和连贯的文本。（默认值：5.0） | 📌 |
| repeat_last_n | 设置模型回看以防止重复的距离。（默认值：64，0 = 禁用，-1 = num_ctx） | 📌 |
| repeat_penalty | 设置惩罚重复的强度。较高的值（如 1.5）将更强烈地惩罚重复，而较低的值（如 0.9）将更宽松。（默认值：1.1） | 📌 |
| seed | 设置用于生成的随机数种子。将其设置为特定数字将使模型为相同的提示生成相同的文本。（默认值：0） | 📌 |
| stop | 设置要使用的停止序列。当遇到此模式时，LLM 将停止生成文本并返回。可以通过在模型文件中指定多个单独的 stop 参数来设置多个停止模式。 | 📌 |
| tfs_z | 尾部自由采样用于减少较不可能的令牌对输出的影响。较高的值（如 2.0）将减少更多影响，而值为 1.0 则禁用此设置。（默认值：1） | 📌 |
| top_k | 降低生成无意义内容的概率。较高的值（如 100）将给出更多样化的答案，而较低的值（如 10）将更保守。（默认值：40） | 📌 |
| min_p | top_p 的替代方案，旨在确保质量和多样性的平衡。参数 p 表示相对于最可能令牌的概率，令牌被考虑的最小概率。例如，当 p=0.05 且最可能的令牌概率为 0.9 时，值小于 0.045 的对数将被过滤掉。（默认值：0.0） | 📌 |

### 参考资料
- [Ollama Modelfile](https://github.com/ollama/ollama/blob/main/docs/modelfile.md#valid-parameters-and-values)

## Google Gemini

📌 **计划支持**

[en-icon]: https://img.shields.io/badge/English-teal?style=flat-square
[zh-hans-icon]: https://img.shields.io/badge/%E7%AE%80%E4%BD%93%E4%B8%AD%E6%96%87-teal?style=flat-square