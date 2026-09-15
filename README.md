# PrismLens - 现代化的图片查看工具

## 项目简介

[![wakatime](https://wakatime.com/badge/user/d04dc633-79ab-469c-af7d-cf8a01ca16eb/project/9627e326-d896-44d8-8598-97df5d8e32e4.svg)](https://wakatime.com/badge/user/d04dc633-79ab-469c-af7d-cf8a01ca16eb/project/9627e326-d896-44d8-8598-97df5d8e32e4)

PrismLens是一款基于Tauri + Vue3 + TypeScript开发的现代化图片查看工具。它不仅提供了强大的图片查看和处理功能，还计划集成社交分享和AI增强特性，致力于打造一个高性能、易用性强的图片管理工具。像棱镜镜头一样，不仅能清晰展现图片的每一个细节，更能展现图片的多彩视角。

> 项目功能、架构与设计细节见 [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)。

## 核心特性

### 1. 图片基础功能

- 🖼️ 多格式图片查看和浏览
- 🔄 基础图片操作（纯视图变换，不修改原文件）
  - 旋转
  - 缩放（按钮 / 滚轮）
  - 拖拽平移
- 🎨 图片格式转换（"另存为"导出，不覆盖原图）
- ✂️ 图片裁剪

### 2. 社交分享功能

- 📱 QQ分享集成
- 🌐 预留其他社交平台分享接口

### 3. AI增强功能（规划中）

- 🤖 智能图片识别
- ⭐ AI图片评分
- 🏷️ 智能标签系统

## 技术栈

### 前端技术

- Vue 3
- TypeScript
- Vite
- Naive UI
- @vueuse/core
- Vue Router
- Pinia

### 后端技术

- Tauri 2 (Rust)
- image-rs
- serde
- thiserror
- tauri-plugin-dialog / tauri-plugin-opener

## 性能设计

- 📊 Rust处理性能密集型任务（命令层通过 `spawn_blocking` 调度，避免阻塞异步运行时）
- 🔄 图片经 Tauri asset 协议直接加载，不经过 IPC 传输字节
- 📱 缩略图列表使用虚拟滚动，避免大目录一次性挂载 DOM
- 💾 图片懒加载与预加载（规划中）

## 开发计划

1. 第一阶段：基础功能实现
   - 图片查看和浏览
   - 基础图片操作
   - 性能优化

2. 第二阶段：社交分享
   - QQ分享功能
   - 其他平台分享接口预留

3. 第三阶段：AI增强
   - 图片识别
   - AI评分系统
   - 智能标签

## 贡献指南

欢迎提交Issue和Pull Request！

## 许可证

MIT License

## 联系方式

[待补充]
