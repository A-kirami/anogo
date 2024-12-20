# 提示词配置指南

在使用 AnoGO 之前，你需要先在大语言模型（LLM）中生成 AnoGO 格式的脚本。为了确保生成的脚本符合预期，我们需要配置“系统提示词”。

> [!TIP]
> 为了更方便地生成 AnoGO 脚本，我们推荐使用第三方应用，例如 [LobeChat](https://github.com/lobehub/lobe-chat)、[ChatBox](https://github.com/Bin-Huang/chatbox) 和 [NextChat](https://github.com/ChatGPTNextWeb/ChatGPT-Next-Web) 等，这些应用提供了便捷的界面和工具来帮助你配置系统提示词。

## 什么是系统提示词（system prompt）？

系统提示词是一种特殊的指令，用于指导 LLM（如 ChatGPT）的行为。它可以帮助你设定 AI 的风格和任务范围，使得 AI 的输出更加符合你的需求。

## 如何使用系统提示词？

AnoGO 已经为你准备了一份系统提示词，只需将其复制到 LLM 的相应设置中即可。以下是具体步骤：

1. 打开 AnoGO 仓库的 [`docs/prompt`](https://github.com/A-kirami/anogo/tree/main/docs/prompt) 文件夹。
2. 找到名为 [`system_prompt.txt`](https://github.com/A-kirami/anogo/blob/main/docs/prompt/system_prompt.txt) 的文件并打开它。
3. 选中所有的文本并复制。

## 在 LLM 中设置系统提示词

不同 LLM 的设置方法可能略有不同，但通常情况下，你需要将复制的系统提示词配置到 LLM 的系统消息或角色/助手设置中。

以下是一些常见 LLM 的设置方法：

### ChatGPT

1. 登录你的 ChatGPT 账户。
2. 点击对话界面的右上角的头像，打开“自定义 ChatGPT”。
3. 将复制的提示词输入其中。
4. 保存设置后，在新的聊天中直接发送你的小说文本。

### 其他 LLM

请参考你使用的 LLM 的官方文档或帮助中心，以获取具体的配置指南。

## 注意事项

- 确保你的系统提示词已经正确设置，否则 LLM 生成的脚本可能不符合预期。
