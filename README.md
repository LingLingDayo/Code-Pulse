# CodePulse (文脉) 🚀

CodePulse 是一款专为大语言模型 (LLM) 设计的高效代码上下文生成工具。通过深度递归分析文件依赖，它能将分散的代码逻辑自动化地汇聚成一份结构化的文本块，是开发者向 AI 发送代码 Prompt 的最佳拍档。

---

## ✨ 核心特性

- **🔍 深度依赖追踪**: 自动解析 `import` / `require` / `use` 等语句，递归获取所有相关源代码（支持自定义深度）。
- **📦 多语言支持**: 完美支持 TypeScript (Vite/Vue), JavaScript, Python 和 Rust。
- **🛡️ 智能过滤**: 自动忽略 `node_modules`, `.git`, `dist`, `target` 等无关目录。
- **📋 一键就绪**: 合并后的上下文带清晰的文件路径标识，一键点击即可复制到剪贴板。
- **⚡ 现代体验**: 基于 Tauri + Vue 3 构建的高性能、极小体积的桌面应用。

---

## 🛠️ 技术栈

- **Frontend**: [Vue 3](https://vuejs.org/) + [TypeScript](https://www.typescriptlang.org/)
- **Styling**: [Tailwind CSS 4.x](https://tailwindcss.com/)
- **Desktop Engine**: [Tauri 2.x](https://tauri.app/)
- **Logic Core**: Rust (High performance dependency analysis)

---

## 🏗️ 快速开始

### 运行开发环境

确保你已经安装了 [Rust](https://www.rust-lang.org/) 环境。

1. **克隆项目**:
   ```bash
   git clone <repo-url>
   cd CodePulse
   ```

2. **安装依赖**:
   ```bash
   npm install
   ```

3. **启动开发服务器**:
   ```bash
   npm run tauri dev
   ```

### 构建应用

```bash
npm run tauri build
```

---

## 📖 使用指南

1. **选择深度**: 在界面上方设置递归搜索的最大深度（推荐 1-3）。
2. **拖放位置**: 将你的代码文件或整个项目文件夹拖入蓝色的虚线区域。
3. **获取上下文**: 解析完成后，合并的代码将呈现在下方的输出框中。
4. **一键复制**: 点击右侧的“一键复制”按钮，直接粘贴到 ChatGPT / Claude / DeepSeek 等 AI 窗口。

---

## 🤝 贡献与反馈

如果你有任何建议、发现 Bug 或想贡献代码，欢迎提交 Issue 或 Pull Request！

---

## 📄 开源协议

MIT License
