# PrismLens 项目功能与架构说明

> 本文档由项目现状梳理生成，并作为本轮重构的依据与记录。

## 1. 项目概览

PrismLens 是一款基于 **Tauri 2 + Vue 3 + TypeScript** 的桌面图片查看 / 处理工具（产品名 `PrismLens`，Tauri identifier `io.github.PrismLens`）。

- Rust 侧负责文件系统 IO 与图像处理（`image` crate），前端 WebView 负责展示与交互。
- 图片不通过 IPC 传输字节，而是通过 Tauri 官方 **asset 协议**直接加载本地文件。
- README 中的 QQ 分享、AI 识别 / 评分 / 标签为后续规划，尚未实现。

## 2. 技术栈（与实际依赖一致）

### 前端

| 依赖 | 用途 |
| --- | --- |
| Vue 3 + TypeScript + Vite | 应用框架与构建 |
| Naive UI | 组件库（`unplugin-vue-components` 自动按需引入） |
| Pinia | 状态管理（`theme` / `images` 两个 store） |
| Vue Router | 路由（`/` 重定向到 `/image`） |
| @vueuse/core | 工具集（`useStorage`、`useVirtualList`） |
| @vicons/* | 图标（antd、ionicons5） |
| Sass | 样式（`abstracts / base / layout / components / pages` 分层） |

### 后端 (Rust)

| 依赖 | 用途 |
| --- | --- |
| tauri 2（`protocol-asset`） | 桌面框架与本地资源协议 |
| image | 图片解码、旋转、缩放、格式转换 |
| serde / serde_json | 结构体序列化 |
| thiserror | 错误类型定义 |
| tauri-plugin-dialog | 文件 / 目录选择对话框 |
| tauri-plugin-opener | 系统打开能力 |

> 注意：README 早期版本提到的 `tokio`、`reqwest` 并未实际使用，本轮已从 README 中移除。

## 3. 目录结构

```plaintext
PrismLens/
├── docs/ARCHITECTURE.md          # 本文档
├── index.html
├── package.json                  # 前端依赖与脚本
├── pnpm-workspace.yaml           # pnpm 设置（原生构建脚本白名单）
├── vite.config.ts                # 自动导入、分包、Tauri dev 端口
├── src/
│   ├── main.ts                   # 入口：Pinia + Router + 全局样式
│   ├── App.vue                   # 根组件：主题注入 + 标题栏 + 路由出口
│   ├── router/index.ts           # 路由表
│   ├── store/
│   │   ├── images.ts             # 图片列表 / 当前图片状态与 actions
│   │   └── theme.ts              # 主题状态与明暗主题覆盖变量
│   ├── components/
│   │   ├── TitleBar.vue          # 无边框窗口自定义标题栏
│   │   ├── ThemeSwitch.vue       # 明暗主题开关
│   │   ├── ImageList.vue         # 缩略图虚拟列表 + 目录选择
│   │   └── ImageOperation.vue    # 旋转 / 格式转换 / 缩放操作面板
│   ├── views/ImageView.vue       # 主页面：大图显示 + 操作 + 列表
│   ├── tauri-types.d.ts          # 与 Rust 结构体对齐的前端类型
│   └── assets/scss/              # SCSS 分层样式
└── src-tauri/
    ├── tauri.conf.json           # 窗口、CSP、asset 协议、打包配置
    ├── capabilities/default.json # 窗口 / dialog / opener 权限
    └── src/
        ├── main.rs               # 入口（windows_subsystem = "windows"）
        ├── lib.rs                # Builder：插件注册 + invoke_handler
        └── core/
            ├── image.rs          # 图像业务逻辑 + 错误/结构体定义
            └── commands.rs       # #[command] 薄封装（spawn_blocking）
```

## 4. 系统架构

### 4.1 前端分层

```plaintext
main.ts
  └─ App.vue (NConfigProvider 主题 + NLayout)
       ├─ TitleBar.vue      —— 调用 @tauri-apps/api/window 控制窗口
       ├─ ThemeSwitch.vue   —— 读写 theme store
       └─ <router-view>
            └─ ImageView.vue
                 ├─ 大图显示：<img :src="convertFileSrc(path)">
                 ├─ ImageOperation.vue —— invoke 图像处理命令
                 └─ ImageList.vue      —— useVirtualList + invoke 目录命令
```

- 所有后端调用集中在 `store/images.ts` 的 actions（`loadDirectoryImages` / `selectImage`），组件只负责交互与消息提示。

### 4.2 后端分层

```plaintext
lib.rs (run)
  └─ core/commands.rs   #[command] async fn + spawn_blocking
       └─ core/image.rs 纯业务函数（可单测）
```

- `image.rs` 定义 `ImageInfo`、`ImageBasicInfo`、`ImageError`（thiserror）与 `Result<T>` 别名。
- 命令层仅做参数接收、`spawn_blocking` 调度与错误 `String` 化，保持薄封装。
- 后端只在用户通过"另存为"显式指定路径时写文件，绝不自动生成副本或覆盖源文件。

### 4.3 数据流

```plaintext
选择文件/目录 (tauri-plugin-dialog)
  → invoke("get_directory_images_command") → Vec<ImageBasicInfo> → store.imageList
  → 点击缩略图 / 选择文件
  → invoke("read_image_info_command") → ImageInfo → store.currentImage
  → <img :src="convertFileSrc(path)"> 经 asset 协议直接读取本地文件
  → 旋转/缩放/拖拽：仅修改 store.view（CSS transform 展示，不写磁盘）
  → 格式转换 / 尺寸调整：save() 弹"另存为" → invoke 命令写入用户指定路径
```

### 4.4 IPC 命令一览

| 命令 | 入参 | 返回 | 说明 |
| --- | --- | --- | --- |
| `read_image_info_command` | `path: String` | `ImageInfo` | 仅读文件头获取尺寸 + 元数据 |
| `read_image_list_command` | `dirPath: String` | `Vec<ImageInfo>` | 完整信息列表（性能差，当前未使用） |
| `get_directory_images_command` | `dirPath: String` | `Vec<ImageBasicInfo>` | 轻量扫描目录（按文件名排序） |
| `convert_image_format_command` | `path, format, outputPath` | `String`（保存路径） | 另存为指定路径；jpg / png / gif / webp / bmp / ico |
| `resize_image_command` | `path, width, height, outputPath` | `String`（保存路径） | 另存为指定路径；Lanczos3 缩放 |

支持查看的扩展名：`jpg jpeg png gif webp bmp ico tiff tif`。

## 5. 功能清单与状态

| 功能 | 状态 |
| --- | --- |
| 选择单张图片并自动扫描同目录列表 | 已完成 |
| 选择目录并列出图片 | 已完成（列表工具栏入口） |
| 缩略图虚拟滚动列表、点击切换大图 | 已完成（当前图片高亮） |
| 图片信息（尺寸 / 格式 / 大小 / 修改时间） | 已完成（显示区左上角浮层） |
| 上一张 / 下一张切换（按钮 + 方向键） | 已完成 |
| 视图旋转 90 / 180 / 270（不改文件） | 已完成 |
| 视图缩放（按钮 / 滚轮）与拖拽平移、适应窗口、重置 | 已完成 |
| 格式转换（"另存为" 6 种格式：PNG / JPG / WebP / GIF / BMP / ICO） | 已完成 |
| 尺寸调整（"另存为"） | 已完成 |
| 明暗主题切换（跟随系统 + 持久化） | 已完成（标题栏按钮） |
| 无边框窗口自定义标题栏 | 已完成（最大化状态与窗口同步） |
| 图片懒加载 / 预加载 | 已完成（列表 `loading="lazy"` + 相邻图片预加载） |
| QQ 分享等多平台分享 | 未实现（规划） |
| AI 识别 / 评分 / 标签 | 未实现（规划） |

## 6. 关键设计

1. **asset 协议加载图片**：`convertFileSrc` 将绝对路径转换为 `asset://`（生产）/ `http://asset.localhost`（开发）地址，配合 `tauri.conf.json` 中 `assetProtocol.scope: ["**"]` 使用；曾尝试自定义协议，已于 `2bd9bf9` 回退为官方方案。
2. **类型对齐**：Rust `ImageBasicInfo` 与前端 `ImageBasicInfo` 一一对应，`ImageInfo extends ImageBasicInfo` 补充 `width / height / format`，避免把轻量对象断言成完整对象。
3. **并发模型**：`#[command] async fn` + `tauri::async_runtime::spawn_blocking`，避免大图解码阻塞异步运行时。
4. **只读浏览 + 显式另存**：旋转/缩放/拖拽是纯视图变换（`store.view` + CSS `transform`），不写磁盘；格式转换与尺寸调整必须通过"另存为"对话框显式指定输出路径，后端不再自动命名副本。
5. **主题体系**：`useStorage("color-scheme")` 持久化，初始值跟随系统；`NConfigProvider` 注入 Naive UI `darkTheme` 与自定义 overrides，标题栏等自定义样式通过 `v-bind` 引用 store。
6. **虚拟列表**：`useVirtualList` 以 200px 横向项宽渲染缩略图，避免大目录一次性挂载 DOM。
7. **视图变换**：显示尺寸按"旋转后外接框适应窗口"计算（`displaySize`），叠加用户缩放比例后以 `translate + rotate` 渲染；指针拖拽平移、滚轮缩放，切换图片自动重置。

## 7. 本轮重构记录

针对现状梳理出的问题，重构项如下：

- [x] 统一图片类型：新增前端 `ImageBasicInfo`，`imageList` 改为轻量类型，`ImageInfo extends ImageBasicInfo`。
- [x] 状态管理收敛：`store/images.ts` 增加 `loadDirectoryImages` / `selectImage` actions，组件不再散落 `invoke`。
- [x] 恢复图像操作入口：`ImageView.vue` 挂载 `ImageOperation`；布局改为 flex 自适应。
- [x] 恢复目录选择入口：`ImageList.vue` 工具栏重新启用"选择图片目录"与计数。
- [x] 修复阻塞 IO：所有命令通过 `spawn_blocking` 执行。
- [x] 后端去重：提取 `SUPPORTED_IMAGE_EXTENSIONS` / `is_supported_image`，目录列表按文件名排序。
- [x] 图像操作语义调整：移除旋转写盘命令与自动命名逻辑（不再产生 `_rotated` / `_resized` 等副本）；旋转/缩放改为纯视图变换，格式转换/尺寸调整改为"另存为"显式输出。
- [x] 命名与路由：组件文件统一 PascalCase；路由改为 `/` 重定向到 `/image`。
- [x] 标题栏清理：移除注释代码，使用 `onResized` 监听同步最大化状态。
- [x] 死代码清理：删除未使用的 `Home.vue`、`tools/message.ts`；README 与实际依赖对齐。
- [x] 工程命名：`package.json` / `Cargo.toml` 包名由 `rview` 统一为 `prismlens`。
- [x] 构建配置：新增 `pnpm-workspace.yaml` 放行 `esbuild` / `@parcel/watcher` 原生构建脚本，移除 package.json 中 pnpm 11 已不再读取的 `pnpm` 字段。

### UI 美化与体验完善（第二轮）

- [x] 设计令牌：新增 `abstracts/_theme.scss`，用 CSS 变量定义明暗两套配色（背景 / 文字 / 边框 / 阴影 / 棋盘格），通过 `<html data-theme>` 切换；`store/theme.ts` 负责写入该属性。
- [x] 主题修复：移除全局 SCSS 中不可用的 `v-bind()`，改用 CSS 变量；清理 Naive UI `Button.textColor` 覆盖导致的按钮文字色异常。
- [x] 标题栏：新增渐变棱镜 Logo 与品牌标题，主题开关内嵌到标题栏，窗口控制按钮统一悬停样式。
- [x] 显示区：棋盘格背景（便于观察透明图片）、左上角图片信息浮层、左右悬浮上一张 / 下一张按钮、重绘空状态。
- [x] 操作栏：按「旋转 / 缩放 / 导出 / 尺寸」分组，补充图标、适应窗口与重置；导出格式扩展为 PNG / JPG / WebP / GIF / BMP / ICO。
- [x] 图片列表：当前图片高亮、文件名浮层、刷新按钮、原生懒加载、横向滚动条。
- [x] 交互增强：方向键切换图片、`+/-` 缩放、`0` 重置；相邻图片预加载。
- [x] 架构收敛：文件 / 目录选择对话框与切换、刷新等命令调用收敛到 `store/images.ts` 的 actions。
- [x] 资源：新增 `public/prism.svg` 作为应用图标。

### 验证情况

- 前端：`pnpm build`（`vue-tsc --noEmit && vite build`）通过。
- 后端：`cargo check` 与 `cargo build` 均通过，产出 `src-tauri/target/debug/prismlens.exe`。
- 单元测试：`cargo test` 通过（`core/image.rs` 6 个用例，覆盖扩展名识别、输出路径校验、图片信息读取、目录扫描过滤与排序、格式转换、尺寸调整）。

### 本机开发环境（已配置）

| 项目 | 位置 / 说明 |
| --- | --- |
| Rust 工具链 | `E:\Rust\rustup`（`RUSTUP_HOME`），stable-x86_64-pc-windows-msvc 1.98.1 |
| Cargo 缓存 | `E:\Rust\cargo`（`CARGO_HOME`，含 crates 缓存；`E:\Rust\cargo\bin` 已加入用户 PATH） |
| VS Build Tools 2022 | `E:\VSBuildTools`（MSVC v14.44 + Windows 11 SDK 26100） |
| Windows SDK | `E:\WindowsKits\10`（VS 共享组件位于 `E:\VSShared`） |
| 编译产物 | `src-tauri\target`（D 盘；debug 约 9GB，可用 `cargo clean` 回收） |

> `RUSTUP_HOME` / `CARGO_HOME` 已写入用户级环境变量，新开终端可直接使用 `cargo` / `rustup` / `pnpm tauri dev`。
> 若 `cargo build` 报 build script `os error 5 拒绝访问`（安全软件锁定刚生成的 exe），执行 `cargo clean -p <crate>` 后重试即可。

## 8. 已知限制与后续计划

1. `read_image_list_command`（完整信息列表）仍保留但性能较差，若需展示分辨率/格式列表，建议按需分页读取。
2. "另存为"生成的文件不会自动加入左侧列表，需要重新扫描目录（后续可在保存成功后刷新列表）。
3. 大图渲染依赖 WebView 解码，超大图片可能内存占用较高（README 中"GPU 纹理缓存"为远期方向）。
4. CSP 当前较宽松（`unsafe-eval`、`*`），后续可按实际域名收紧。
5. 分享与 AI 能力尚未开工，接口预留位置未定义。
6. `image.rs` 已有基础单元测试（`cargo test`）；前端组件仍无自动化测试，后续可引入 Vitest + @vue/test-utils 覆盖 store 与组件逻辑。
