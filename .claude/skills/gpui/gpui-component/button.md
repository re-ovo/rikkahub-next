# Button 组件

## 概述

Button 是 gpui-component 中的按钮组件，提供丰富的样式变体、大小选项和交互功能。支持多种按钮变体（Primary、Secondary、Danger、Warning、Success、Info、Ghost、Link、Text），可自定义样式、加载状态、禁用状态、选中状态等。适用于各种UI场景的按钮交互。

## API 定义

### 核心结构

```rust
pub struct Button {
    id: ElementId,
    base: Stateful<Div>,
    style: StyleRefinement,
    icon: Option<Icon>,
    label: Option<SharedString>,
    children: Vec<AnyElement>,
    disabled: bool,
    pub(crate) selected: bool,
    variant: ButtonVariant,
    rounded: ButtonRounded,
    outline: bool,
    border_corners: Corners<bool>,
    border_edges: Edges<bool>,
    dropdown_caret: bool,
    size: Size,
    compact: bool,
    tooltip: Option<(SharedString, Option<(Rc<Box<dyn Action>>, Option<SharedString>)>)>,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
    on_hover: Option<Rc<dyn Fn(&bool, &mut Window, &mut App)>>,
    loading: bool,
    loading_icon: Option<Icon>,
    tab_index: isize,
    tab_stop: bool,
}
```

### 按钮变体枚举

```rust
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    Primary,
    #[default]
    Secondary,
    Danger,
    Info,
    Success,
    Warning,
    Ghost,
    Link,
    Text,
    Custom(ButtonCustomVariant),
}
```

### 圆角半径配置

```rust
#[derive(Default, Clone, Copy)]
pub enum ButtonRounded {
    None,
    Small,
    #[default]
    Medium,
    Large,
    Size(Pixels),
}
```

### 自定义按钮样式

```rust
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ButtonCustomVariant {
    color: Hsla,              // 背景颜色
    foreground: Hsla,         // 文字颜色
    border: Hsla,             // 边框颜色
    shadow: bool,             // 是否显示阴影
    hover: Hsla,              // 悬停状态背景颜色
    active: Hsla,             // 激活状态背景颜色
}
```

## 主要方法/属性

| 方法 | 参数类型 | 说明 |
|------|---------|------|
| `new(id)` | `impl Into<ElementId>` | 创建新的按钮实例，必须提供唯一的元素ID |
| `label(label)` | `impl Into<SharedString>` | 设置按钮的文本标签 |
| `icon(icon)` | `impl Into<Icon>` | 设置按钮的图标，支持 IconName 或 Icon 对象 |
| `disabled(disabled)` | `bool` | 设置按钮是否禁用（来自 Disableable trait）|
| `selected(selected)` | `bool` | 设置按钮是否处于选中状态（来自 Selectable trait）|
| `with_size(size)` | `impl Into<Size>` | 设置按钮大小（来自 Sizable trait），支持 XSmall、Small、Medium、Large、Size(px) |
| `outline()` | - | 启用边框样式（outline mode） |
| `rounded(rounded)` | `impl Into<ButtonRounded>` | 设置圆角半径 |
| `loading(loading)` | `bool` | 设置按钮是否处于加载状态 |
| `loading_icon(icon)` | `impl Into<Icon>` | 自定义加载时显示的图标，默认为 Spinner |
| `compact()` | - | 启用紧凑模式，减少内边距 |
| `on_click(handler)` | `impl Fn(&ClickEvent, &mut Window, &mut App) + 'static` | 添加点击事件处理器 |
| `on_hover(handler)` | `impl Fn(&bool, &mut Window, &mut App) + 'static` | 添加悬停事件处理器 |
| `tooltip(tooltip)` | `impl Into<SharedString>` | 设置工具提示文本 |
| `tooltip_with_action(tooltip, action, context)` | `&str, &dyn Action, Option<&str>` | 设置工具提示并显示快捷键绑定 |
| `dropdown_caret(caret)` | `bool` | 在按钮末尾显示下拉箭头 |
| `tab_index(index)` | `isize` | 设置Tab键导航顺序，默认为 0 |
| `tab_stop(stop)` | `bool` | 设置是否可通过Tab键聚焦，默认为 true |

### 样式变体方法（ButtonVariants trait）

所有这些方法都是 Builder 模式，链式调用：

| 方法 | 说明 |
|------|------|
| `primary()` | 主色调按钮 - 用于主要操作 |
| `secondary()` | 次色调按钮 - 默认样式 |
| `danger()` | 危险色调按钮 - 用于删除、重置等破坏性操作 |
| `warning()` | 警告色调按钮 - 用于警告操作 |
| `success()` | 成功色调按钮 - 用于成功状态 |
| `info()` | 信息色调按钮 - 用于信息提示 |
| `ghost()` | 幽灵按钮 - 透明背景，仅文字和边框 |
| `link()` | 链接样式按钮 - 看起来像超链接 |
| `text()` | 文本样式按钮 - 最小化样式，看起来像普通文本 |
| `custom(style)` | 自定义样式按钮 - 完全自定义颜色方案 |

### ButtonCustomVariant 方法

用于构建自定义按钮样式：

| 方法 | 参数 | 说明 |
|------|------|------|
| `new(cx)` | `&App` | 创建新的自定义样式，使用主题透明色初始化 |
| `color(color)` | `Hsla` | 设置背景颜色，默认为透明 |
| `foreground(color)` | `Hsla` | 设置文字颜色，默认为主题前景色 |
| `border(color)` | `Hsla` | 设置边框颜色，默认为透明 |
| `hover(color)` | `Hsla` | 设置悬停时的背景颜色 |
| `active(color)` | `Hsla` | 设置激活时的背景颜色 |
| `shadow(shadow)` | `bool` | 设置是否显示阴影，默认为 false |

## 使用示例

### 基础用法

```rust
use gpui_component::button::{Button, ButtonVariants as _};
use gpui::{IntoElement, ParentElement};

// 创建一个简单的主色调按钮
div()
    .child(
        Button::new("my-button")
            .primary()
            .label("Click me")
            .on_click(|_, _, _| {
                println!("Button clicked!");
            })
    )
```

### 不同的按钮变体

```rust
// Primary 按钮 - 主要操作
Button::new("primary-btn")
    .primary()
    .label("Save")
    .on_click(|_, _, _| { /* save */ })

// Danger 按钮 - 删除操作
Button::new("delete-btn")
    .danger()
    .label("Delete")
    .on_click(|_, _, _| { /* delete */ })

// Ghost 按钮 - 次要操作
Button::new("cancel-btn")
    .ghost()
    .label("Cancel")
    .on_click(|_, _, _| { /* cancel */ })

// Link 按钮 - 看起来像链接
Button::new("more-btn")
    .link()
    .label("Learn more")
    .on_click(|_, _, _| { /* navigate */ })

// Text 按钮 - 最小化样式
Button::new("text-btn")
    .text()
    .label("Optional action")
    .on_click(|_, _, _| { /* action */ })
```

### 按钮大小

```rust
use gpui_component::Sizable as _;

// XSmall 按钮
Button::new("xsmall-btn")
    .label("Small")
    .xsmall()

// Small 按钮
Button::new("small-btn")
    .label("Small")
    .small()

// Medium 按钮 (默认)
Button::new("medium-btn")
    .label("Medium")

// Large 按钮
Button::new("large-btn")
    .label("Large")
    .large()

// 自定义大小
use gpui::px;
Button::new("custom-btn")
    .label("Custom")
    .size(px(48.))
```

### 带图标的按钮

```rust
use gpui_component::{Icon, IconName};

// 带图标的按钮
Button::new("with-icon")
    .primary()
    .label("Confirm")
    .icon(IconName::Check)
    .on_click(|_, _, _| { /* confirm */ })

// 仅图标按钮
Button::new("icon-only")
    .icon(IconName::Search)
    .on_click(|_, _, _| { /* search */ })

// 自定义图标大小
Button::new("custom-icon")
    .label("Download")
    .icon(Icon::new(IconName::Download).lg())
    .on_click(|_, _, _| { /* download */ })
```

### 禁用和选中状态

```rust
use gpui_component::{Disableable as _, Selectable as _};

// 禁用按钮
Button::new("disabled-btn")
    .label("Disabled")
    .disabled(true)

// 选中状态
Button::new("selected-btn")
    .label("Selected")
    .selected(true)

// 禁用且具有其他状态
Button::new("complex-btn")
    .label("Complex")
    .disabled(!is_enabled)
    .selected(is_selected)
    .danger()
```

### 加载状态

```rust
// 启用加载状态，显示默认 Spinner
Button::new("loading-btn")
    .primary()
    .label("Saving")
    .loading(true)
    .disabled(true)

// 自定义加载图标
Button::new("loading-custom")
    .label("Loading")
    .loading(true)
    .loading_icon(IconName::LoaderCircle)
    .on_click(|_, _, _| { /* save */ })
```

### 工具提示（Tooltip）

```rust
// 简单工具提示
Button::new("tooltip-btn")
    .label("Hover me")
    .tooltip("This is a helpful tip")
    .on_click(|_, _, _| { /* action */ })

// 带快捷键的工具提示
use gpui::Action;

#[derive(Clone, Action)]
struct MyAction;

Button::new("tooltip-action")
    .label("Save")
    .tooltip_with_action("Save file", &MyAction, Some("editor"))
    .on_click(|_, _, _| { /* save */ })
```

### 样式定制

```rust
use gpui_component::button::ButtonCustomVariant;

// 自定义样式按钮
let custom_style = ButtonCustomVariant::new(cx)
    .color(cx.theme().magenta)
    .foreground(cx.theme().primary_foreground)
    .border(cx.theme().magenta)
    .hover(cx.theme().magenta.opacity(0.1))
    .active(cx.theme().magenta);

Button::new("custom-btn")
    .label("Custom Styled")
    .custom(custom_style)
    .on_click(|_, _, _| { /* action */ })
```

### Outline 模式

```rust
// Outline 模式 - 边框样式
Button::new("outline-primary")
    .primary()
    .outline()
    .label("Outline Primary")

Button::new("outline-danger")
    .danger()
    .outline()
    .label("Outline Danger")

// Outline 与自定义样式
Button::new("outline-custom")
    .custom(custom_variant)
    .outline()
    .label("Custom Outline")
```

### 圆角和紧凑模式

```rust
use gpui_component::button::ButtonRounded;
use gpui::px;

// 设置圆角
Button::new("rounded-small")
    .label("Small Radius")
    .rounded(ButtonRounded::Small)

Button::new("rounded-large")
    .label("Large Radius")
    .rounded(ButtonRounded::Large)

// 完全圆形（用于图标按钮）
Button::new("rounded-full")
    .label("Circle")
    .rounded(px(999.))

// 紧凑模式 - 减少内边距
Button::new("compact-btn")
    .label("Compact")
    .compact()
    .small()
```

### 下拉菜单样式

```rust
// 带下拉箭头
Button::new("dropdown-btn")
    .label("Options")
    .dropdown_caret(true)
    .on_click(|_, _, _| { /* show menu */ })
```

### 事件处理

```rust
// 点击事件处理
Button::new("click-btn")
    .label("Click me")
    .on_click(|event, window, cx| {
        println!("Clicked at position: {:?}", event.position);
        // 处理点击事件
    })

// 悬停事件处理
Button::new("hover-btn")
    .label("Hover me")
    .on_hover(|is_hovering, _, _| {
        if is_hovering {
            println!("Mouse entered");
        } else {
            println!("Mouse left");
        }
    })

// 组合事件
Button::new("multi-event")
    .label("Complex")
    .on_click(|_, _, _| { /* handle click */ })
    .on_hover(|_, _, _| { /* handle hover */ })
    .tooltip("More info here")
```

### Tab 键导航

```rust
// 设置 Tab 导航顺序
Button::new("first-btn")
    .label("First")
    .tab_index(0)

Button::new("second-btn")
    .label("Second")
    .tab_index(1)

// 禁用 Tab 导航
Button::new("no-tab")
    .label("Skip Tab")
    .tab_stop(false)
```

### 自定义内容

```rust
use gpui::{IntoElement, ParentElement};
use gpui_component::{h_flex, IconName};

// 使用 child 添加自定义内容
Button::new("custom-content")
    .child(
        h_flex()
            .items_center()
            .gap_2()
            .child("Custom")
            .child(IconName::ChevronDown)
    )
    .on_click(|_, _, _| { /* action */ })
```

## Button 状态变化

Button 支持以下状态的自动样式变化：

| 状态 | 说明 |
|------|------|
| Normal | 默认状态 |
| Hover | 鼠标悬停时的样式 |
| Active | 鼠标按下时的样式 |
| Focused | 获得焦点时显示焦点环 |
| Selected | 选中时的高亮样式 |
| Disabled | 禁用时的灰显样式 |
| Loading | 加载中时的样式（禁用交互，显示加载图标） |

## 常见用法模式

### 确认对话框中的按钮组

```rust
use gpui_component::h_flex;

h_flex()
    .gap_3()
    .child(
        Button::new("cancel")
            .ghost()
            .label("Cancel")
            .on_click(|_, _, _| { /* close */ })
    )
    .child(
        Button::new("confirm")
            .danger()
            .label("Delete")
            .on_click(|_, _, _| { /* delete */ })
    )
```

### 表单操作按钮

```rust
// 提交按钮
Button::new("submit")
    .primary()
    .label("Submit")
    .loading(is_submitting)
    .disabled(is_submitting || !is_form_valid)
    .on_click(|_, _, _| { /* submit form */ })

// 重置按钮
Button::new("reset")
    .ghost()
    .label("Reset")
    .on_click(|_, _, _| { /* reset form */ })
```

### 工具栏按钮

```rust
use gpui_component::h_flex;

h_flex()
    .gap_1()
    .child(
        Button::new("bold")
            .ghost()
            .compact()
            .icon(IconName::Bold)
            .selected(is_bold)
            .on_click(|_, _, _| { /* toggle bold */ })
    )
    .child(
        Button::new("italic")
            .ghost()
            .compact()
            .icon(IconName::Italic)
            .selected(is_italic)
            .on_click(|_, _, _| { /* toggle italic */ })
    )
```

## 注意事项

1. **必须提供唯一 ID**: Button 必须用唯一的 ElementId，用于焦点管理和状态跟踪。

2. **禁用时不处理事件**: 禁用的按钮不会触发 `on_click` 或 `on_hover` 事件，设置 `disabled(true)` 会自动防止事件处理。

3. **加载状态自动禁用**: 当 `loading(true)` 时，按钮会自动禁用点击功能，避免重复提交。

4. **Icon 按钮特殊处理**: 当按钮没有 `label` 和 `children` 时，按钮会自动进入"Icon Button"模式，以小方形显示（默认大小为 32px）。

5. **Outline 模式边框**: Outline 模式下，某些变体（Primary、Danger等）会根据背景色自动调整边框颜色，使其更加清晰。

6. **Ghost 和 Link 无内边距**: Ghost 和 Link 变体会自动减少内边距，更加轻量化。

7. **Text 变体最小化**: Text 变体不显示背景和边框，仅显示文字，最适合用作次要操作。

8. **加载图标替换**: 当 `loading(true)` 时，`icon` 会被 `loading_icon`（默认为 Spinner）替换。

9. **焦点样式**: 按钮获得焦点时会显示焦点环（focus ring），这可以通过 GPUI 的主题配置调整。

10. **事件处理器签名**: Click 和 Hover 事件处理器都需要 `'static` 生命周期，通常使用闭包或通过 context listener 实现。

11. **Hover 事件逻辑**: `on_hover` 处理器在禁用或加载时不会被调用，只在正常状态下响应。

12. **自定义样式保持一致性**: 使用 `ButtonCustomVariant` 时，应确保 `hover` 和 `active` 颜色能够清晰区分于 `color`，以提供良好的用户反馈。

## 相关组件

- **ButtonGroup**: 按钮组件，用于将多个按钮组织在一起
- **DropdownButton**: 下拉菜单按钮
- **Tooltip**: 工具提示组件，通常与 Button 配合使用
- **Icon**: 图标组件，与 Button 图标功能配合使用
