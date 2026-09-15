# PrismLens - 现代化的图片查看工具

## 项目简介

[![wakatime](https://wakatime.com/badge/user/d04dc633-79ab-469c-af7d-cf8a01ca16eb/project/9627e326-d896-44d8-8598-97df5d8e32e4.svg)](https://wakatime.com/badge/user/d04dc633-79ab-469c-af7d-cf8a01ca16eb/project/9627e326-d896-44d8-8598-97df5d8e32e4)

PrismLens 是一款基于 Tauri 2 + Vue 3 + TypeScript 开发的现代化图片查看工具。它采用「Rust 负责文件系统 IO 与图像处理、WebView 负责展示与交互」的架构，图片通过 Tauri 官方 asset 协议直接加载本地文件，不经 IPC 传输字节，兼顾性能与易用性。像棱镜镜头一样，不仅能清晰展现图片的每一个细节，更能展现图片的多彩视角。

> 项目功能、架构与设计细节见 [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)。

## 核心特性

### 1. 图片浏览

- 多格式支持：`jpg` `jpeg` `png` `gif` `webp` `bmp` `ico` `tiff` `tif`
- 打开单张图片时自动扫描同目录，生成缩略图列表
- 缩略图虚拟滚动列表，当前图片高亮，点击即可切换
- 上一张 / 下一张切换（悬浮按钮 + 方向键）
- 图片信息展示：尺寸、格式、大小、修改时间
- 列表懒加载 + 相邻图片预加载

### 2. 图片操作（纯视图变换，不修改原文件）

- 旋转 90° / 180° / 270°
- 缩放（按钮 / 滚轮 / 快捷键）
- 拖拽平移、适应窗口、一键重置
- 棋盘格背景，便于观察透明图片

### 3. 导出

- 格式转换（"另存为"）：`PNG` `JPG` `WebP` `GIF` `BMP` `ICO`
- 尺寸调整（Lanczos3 缩放，"另存为"）
- 仅在用户显式指定路径时写文件，绝不覆盖或自动生成副本

### 4. 界面

- 明暗主题切换，跟随系统并持久化
- 无边框窗口 + 自定义标题栏（渐变 Logo、主题开关、窗口控制）
- 键盘快捷键：`←` / `→` 切换图片，`+` / `-` 缩放，`0` 重置视图

### 5. 规划中

- ✂️ 图片裁剪
- 📱 QQ 分享等多平台分享
- 🤖 AI 图片识别 / 评分 / 智能标签

## 技术栈

### 前端技术

- Vue 3
- TypeScript
- Vite
- Naive UI
- @vueuse/core
- Vue Router
- Pinia
- Sass（CSS 变量主题令牌）

### 后端技术

- Tauri 2 (Rust)
- image-rs
- serde
- thiserror
- tauri-plugin-dialog / tauri-plugin-opener

## 性能设计

- 📊 Rust 处理性能密集型任务（命令层通过 `spawn_blocking` 调度，避免阻塞异步运行时）
- 🔄 图片经 Tauri asset 协议直接加载，不经过 IPC 传输字节
- 📱 缩略图列表使用虚拟滚动，避免大目录一次性挂载 DOM
- 💾 列表图片懒加载，并预加载当前图片的相邻图片

## 开发与构建

```bash
# 安装依赖
pnpm install

# 开发模式（启动 Vite + Tauri 窗口）
pnpm tauri dev

# 仅构建前端
pnpm build

# 打包桌面安装包（NSIS / MSI 等）
pnpm tauri build

# 后端单元测试（在 src-tauri 目录）
cd src-tauri && cargo test
```

## 开发计划

1. 第一阶段：基础功能实现
   - 图片查看和浏览 —— 已完成
   - 基础图片操作 —— 已完成
   - 性能优化 —— 已完成（虚拟列表 / 懒加载 / 预加载）
2. 第二阶段：社交分享
   - QQ 分享功能
   - 其他平台分享接口预留
3. 第三阶段：AI 增强
   - 图片识别
   - AI 评分系统
   - 智能标签

## 贡献指南

欢迎提交 Issue 和 Pull Request！

## 许可证

MIT License

## 联系方式

[待补充]
