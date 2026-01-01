---
name: GPUI
description: GPUI is a fast, productive UI framework for Rust from the creators of Zed. This document documents the usage of various GPUI APIs, including GPUI Component usage.
---

# GPUI

GPUI 是Zed编辑器的核心库,用于构建跨平台高性能桌面应用程序

## 核心API

查看 [core.md](core.md) 文件获取GPUI的核心API使用方法.


## gpui-component

`gpui-component` 是为GPUI设计的类似shadcn/ui风格的组件库.

### 核心系统

- [Theme](gpui-component/theme.md) - 主题系统，支持明暗主题切换、自定义主题和热更新

### 组件文档

- [Button](gpui-component/button.md) - 按钮组件，支持多种样式变体和状态
- [Input](gpui-component/input.md) - 文本输入组件，支持单行/多行/代码编辑器模式
- [Select](gpui-component/select.md) - 下拉选择组件，支持搜索、分组、自定义渲染
- [Sidebar](gpui-component/sidebar.md) - 侧边栏组件，支持可收缩、菜单项、页眉页脚
- [VirtualList](gpui-component/virtual_list.md) - 虚拟列表组件，高性能渲染大量不同高度的项目

