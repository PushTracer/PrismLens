# SCSS 项目结构说明

## 目录结构

```plaintext
src/assets/scss/
├── abstracts/              # 抽象层
│   ├── _variables.scss     # 变量定义
│   ├── _functions.scss     # 函数定义
│   ├── _mixins.scss        # 混合器定义
│   └── _placeholders.scss  # 占位符定义
├── base/                   # 基础层
│   ├── _reset.scss        # 重置样式
│   ├── _typography.scss   # 排版样式
│   └── _animations.scss   # 动画定义
├── components/            # 组件层
│   ├── _imagelist.scss    # 图片列表样式
│   ├── _cards.scss        # 卡片样式
│   └── _modals.scss       # 模态框样式
├── layout/                # 布局层
│   ├── _titlebar.scss     # 标题栏样式
│   └── _app.scss          # 主内容样式
├── pages/                 # 页面层
│   ├── _home.scss         # 首页特定样式
│   └── _imageview.scss    # 图片窗口样式
└── main.scss              # 主文件，导入所有模块
 */
```

## 例子

```scss
// 变量使用示例
$primary-color: #4a90e2;

// Mixin 使用示例
@mixin flex-center {
  display: flex;
  justify-content: center;
  align-items: center;
}

// BEM 命名示例
.card {
  &__header {
    // ...
  }
  
  &__content {
    // ...
  }
  
  &--active {
    // ...
  }
}
```
