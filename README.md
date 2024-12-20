<p align="center">
  <a href="https://github.com/A-kirami/anogo">
    <img src="./public/anogo.webp" alt="AnoGO Logo" width="180">
  </a>
</p>

<div align="center">

# AnoGO

🍬WebGAL 脚本辅助生成器🍬

</div>

<p align="center">
  <a href="https://github.com/A-kirami/anogo/actions/workflows/build.yml" target="__blank">
    <img src="https://github.com/A-kirami/anogo/actions/workflows/build.yml/badge.svg?branch=main&event=push"
      alt="Github Actions">
  </a>
  <br />
  <a href="https://github.com/A-kirami/anogo/releases/latest" target="__blank">
    <img
      src="https://img.shields.io/github/v/release/A-kirami/anogo?include_prereleases&&color=70aeff&style=social"
      alt="Release Version">
  </a>
  <a href="https://github.com/A-kirami/anogo/stargazers" target="__blank">
    <img alt="GitHub stars" src="https://img.shields.io/github/stars/A-kirami/anogo?style=social">
  </a>
  <a href="https://github.com/A-kirami/anogo/releases" target="__blank">
    <img alt="GitHub downloads"
      src="https://img.shields.io/github/downloads/A-kirami/anogo/total?style=social">
  </a>
</p>

**AnoGO** 是一款 WebGAL 的 MyGO 二创辅助工具，旨在助力创作者高效地将小说文本转换为 WebGAL 脚本。通过结合先进的大语言模型（LLM）技术，AnoGO 简化了创作流程，显著提升创作速度，让 WebGAL 的制作变得轻松简单。

AnoGO 的核心功能是脚本转换，它提供了一个图形界面，并利用 AI 技术辅助创作。用户可以通过 LLM 生成 AnoGO 脚本，然后导入 AnoGO 进行进一步的处理。

## 🌟 特性

- **开箱即用**：无需复杂配置，简单几步将小说文本转换为 WebGAL 脚本。
- **图形化界面**：直观易用的图形化界面设计，无需编程基础也能轻松上手。
- **高效创作**：借助 AI 智能辅助，快速完成 WebGAL 脚本制作，提升创作效率。

## 📦 安装与使用

### 安装

1. 访问项目的 [Releases 页面](https://github.com/A-kirami/anogo/releases/latest)。
2. 下载适用于你操作系统的安装包。
3. 运行安装程序，按照提示完成安装。

### 使用

#### 第一步：配置 LLM

> [!IMPORTANT]
> 在开始之前，确保已正确配置 LLM。请参考 `docs/prompt` 文件夹中的 [提示词配置指南](https://github.com/A-kirami/anogo/tree/main/docs/prompt) 以获取详细的配置流程。

1. 确保已根据 `docs/prompt` 文件夹中的指南配置了LLM。

#### 第二步：准备脚本

> [!TIP]
> 不同 LLM 生成的脚本可能存在差异，建议在多个 LLM 中尝试，并多次生成，挑选最满意的结果。

1. 将你的小说文本发送给 LLM。
2. 复制 LLM 生成的 AnoGO 脚本。

#### 第三步：导入脚本

> [!IMPORTANT]
> 请仔细检查 LLM 生成的脚本是否正确，并手动修改任何错误之处。

1. 打开 AnoGO 应用。
2. 在 AnoGO 中选择 WebGAL 所在的目录以及正在制作的游戏。
3. 将复制的脚本粘贴到指定区域。

#### 第四步：生成 WebGAL 脚本

> [!NOTE]
> 生成的脚本仅为初步草稿，仍需手动添加背景、调整人物位置和表情等细节。

1. 点击“生成脚本”按钮。
2. 导出并查看生成的 WebGAL 脚本。

## 🤝 贡献

我们欢迎任何形式的贡献！如果你有兴趣为 AnoGO 贡献代码或文档，请参阅[贡献指南](./.github/CONTRIBUTING.md)。

## 📄 许可证

Code: AGPL-3.0 - 2024 - Akirami
