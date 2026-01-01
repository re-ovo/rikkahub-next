# Input 组件

## 概述

`Input` 是一个功能完整的文本输入组件，支持单行和多行输入模式。它绑定到 `InputState` 实体，提供了丰富的交互功能，包括清除按钮、掩码切换、加载状态、前缀和后缀元素等。该组件适用于表单输入、搜索框、代码编辑器等多种场景。

## API 定义

### Input 结构

```rust
pub struct Input {
    state: Entity<InputState>,
    style: StyleRefinement,
    size: Size,
    prefix: Option<AnyElement>,
    suffix: Option<AnyElement>,
    height: Option<DefiniteLength>,
    appearance: bool,
    cleanable: bool,
    mask_toggle: bool,
    disabled: bool,
    bordered: bool,
    focus_bordered: bool,
    tab_index: isize,
    selected: bool,
}
```

### InputState 结构

`InputState` 是Input组件的内部状态管理实体，包含：

- `text: String` - 输入框的文本内容
- `disabled: bool` - 是否禁用输入
- `size: Size` - 输入框大小（Small/Medium/Large）
- `mode: InputMode` - 输入模式（SingleLine/MultiLine/CodeEditor）
- `focus_handle: FocusHandle` - 焦点管理句柄
- `masked: bool` - 密码掩码状态
- `loading: bool` - 加载状态
- `search_panel: Option<AnyElement>` - 搜索面板
- `scroll_handle: ScrollHandle` - 滚动句柄
- `scroll_size: Size<Pixels>` - 滚动尺寸

## 主要方法

| 方法 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `new()` | `state: &Entity<InputState>` | `Self` | 创建新的Input实例，绑定到指定的InputState |
| `prefix()` | `prefix: impl IntoElement` | `Self` | 设置输入框左侧的前缀元素（如图标） |
| `suffix()` | `suffix: impl IntoElement` | `Self` | 设置输入框右侧的后缀元素 |
| `h()` | `height: impl Into<DefiniteLength>` | `Self` | 设置多行输入框的高度 |
| `h_full()` | - | `Self` | 设置多行输入框为100%高度 |
| `appearance()` | `appearance: bool` | `Self` | 控制输入框的外观样式（边框、背景），默认true |
| `bordered()` | `bordered: bool` | `Self` | 设置是否显示边框，默认true |
| `focus_bordered()` | `bordered: bool` | `Self` | 设置获得焦点时是否显示边框，默认true |
| `cleanable()` | `cleanable: bool` | `Self` | 设置是否显示清除按钮，默认false |
| `mask_toggle()` | - | `Self` | 启用密码掩码切换按钮（眼睛图标） |
| `disabled()` | `disabled: bool` | `Self` | 禁用输入框 |
| `tab_index()` | `index: isize` | `Self` | 设置Tab键顺序，默认0 |
| `with_size()` | `size: impl Into<Size>` | `Self` | 设置输入框大小（Small/Medium/Large） |
| `selected()` | `selected: bool` | `Self` | 设置是否被选中 |

## InputState 关键方法

| 方法 | 说明 |
|------|------|
| `text()` | 获取当前输入的文本内容 |
| `set_text()` | 设置输入框的文本内容 |
| `set_mode()` | 设置输入模式（单行/多行/代码编辑器） |
| `set_masked()` | 设置密码掩码状态 |
| `clean()` | 清空输入框内容 |
| `focus()` | 获取焦点 |
| `set_disabled()` | 设置禁用状态 |

## 使用示例

### 基础用法

```rust
use gpui::{prelude::*, cx, Entity};
use gpui_component::input::{Input, InputState};

// 在你的视图中创建InputState
let input_state = cx.new(|cx| {
    InputState::new(cx)
});

// 渲染Input组件
div().child(
    Input::new(&input_state)
        .with_size(Size::Medium)
)
```

### 带清除按钮的搜索框

```rust
Input::new(&input_state)
    .with_size(Size::Medium)
    .placeholder("搜索...")
    .prefix(Icon::new(IconName::Search))
    .cleanable(true)  // 显示清除按钮
    .bordered(true)
```

### 密码输入框

```rust
Input::new(&input_state)
    .with_size(Size::Medium)
    .mask_toggle()  // 启用密码掩码切换按钮
    .bordered(true)
```

### 多行输入框（文本区域）

```rust
let state = cx.new(|cx| {
    let mut state = InputState::new(cx);
    state.set_mode(InputMode::MultiLine);
    state
});

Input::new(&state)
    .with_size(Size::Medium)
    .h(px(200.))  // 设置高度
    .appearance(true)
    .bordered(true)
```

### 代码编辑器模式

```rust
let state = cx.new(|cx| {
    let mut state = InputState::new(cx);
    state.set_mode(InputMode::CodeEditor);
    state
});

Input::new(&state)
    .with_size(Size::Medium)
    .h_full()  // 充满容器
    .appearance(true)
    .bordered(true)
```

### 带前缀和后缀的输入框

```rust
Input::new(&input_state)
    .with_size(Size::Medium)
    .prefix(Icon::new(IconName::Mail))
    .suffix(div().text("@example.com"))
    .bordered(true)
```

### 禁用状态

```rust
Input::new(&input_state)
    .with_size(Size::Medium)
    .disabled(true)  // 禁用输入
    .appearance(true)
    .bordered(false)  // 禁用状态下可隐藏边框
```

### 无边框输入框

```rust
Input::new(&input_state)
    .with_size(Size::Small)
    .appearance(false)  // 无背景和边框
    .bordered(false)
    .focus_bordered(false)
```

### 完整的表单字段示例

```rust
v_flex()
    .gap_2()
    .child(
        Label::new("邮箱地址")
    )
    .child(
        Input::new(&email_state)
            .with_size(Size::Medium)
            .placeholder("输入你的邮箱")
            .prefix(Icon::new(IconName::Mail))
            .cleanable(true)
            .bordered(true)
    )
    .child(
        Label::new("密码")
    )
    .child(
        Input::new(&password_state)
            .with_size(Size::Medium)
            .placeholder("输入密码")
            .mask_toggle()  // 显示密码切换
            .bordered(true)
    )
```

## InputMode 说明

Input支持三种输入模式：

- **SingleLine** - 单行输入，适用于文本框、搜索框等
- **MultiLine** - 多行输入，适用于文本区域、评论框等
- **CodeEditor** - 代码编辑器模式，具有行号、语法高亮等功能

```rust
// 在InputState中设置模式
state.update(cx, |state, cx| {
    state.set_mode(InputMode::MultiLine);
});
```

## Size 尺寸

| 尺寸 | 说明 |
|------|------|
| `Size::Small` | 小尺寸，适用于紧凑布局 |
| `Size::Medium` | 中等尺寸（默认），适用于大多数场景 |
| `Size::Large` | 大尺寸，适用于突出的输入框 |

## 键盘快捷键支持

Input 组件自动支持标准的键盘快捷键：

- **编辑操作**：Backspace、Delete、Ctrl+A、Ctrl+X、Ctrl+C、Ctrl+V
- **撤销/重做**：Ctrl+Z、Ctrl+Shift+Z
- **移动光标**：Arrow Keys、Home、End、Ctrl+Left/Right
- **选择文本**：Shift+Arrow Keys、Shift+Home/End
- **多行模式**：Tab/Shift+Tab（缩进/反缩进）、Page Up/Down
- **搜索**：Ctrl+F（在代码编辑器模式）

## 事件处理

InputState 通过标准的GPUI事件系统触发以下事件：

- **文本变化** - 当输入内容改变时
- **焦点变化** - 获得/失去焦点时
- **按键事件** - 处理各种键盘输入
- **鼠标事件** - 点击、选择等交互

```rust
// 监听输入框的值变化
input_state.update(cx, |state, cx| {
    let current_text = state.text();
    println!("当前输入: {}", current_text);
});
```

## 样式定制

Input 组件实现了 `Styled` trait，支持GPUI的所有样式方法：

```rust
Input::new(&input_state)
    .with_size(Size::Medium)
    .p_3()
    .rounded_lg()
    .border_2()
    .border_color(cx.theme().accent)
    .text_sm()
```

## 注意事项

1. **必需的InputState** - 每个Input实例都需要绑定到一个 Entity<InputState>，不能直接创建状态，必须在GPUI上下文中创建

2. **多行输入的高度** - 多行模式下必须明确设置高度（使用 `h()` 或 `h_full()`），否则输入框可能无法正确显示

3. **焦点管理** - Input会自动管理焦点，但在某些情况下需要手动调用 `focus()` 方法

4. **性能考虑** - 对于代码编辑器模式的大量文本，应注意性能影响

5. **样式继承** - Input组件会继承父容器的一些样式属性，需要谨慎配置

6. **掩码切换** - mask_toggle() 按钮仅在输入框获得焦点时显示眼睛图标

7. **清除按钮** - cleanable() 选项仅在输入框非空时显示清除按钮

8. **禁用状态** - 禁用输入框时，所有交互功能都会被禁用，包括清除按钮和掩码切换

## 相关组件

- [Label](label.md) - 标签组件，配合Input使用
- [Button](button.md) - 按钮组件，可用于清除或提交
- [Icon](icon.md) - 图标组件，用于前缀/后缀
