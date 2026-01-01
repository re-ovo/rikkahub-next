# Sidebar 组件

## 概述

`Sidebar` 是一个功能完整的侧边栏组件，支持可收缩/展开、分组菜单、页眉页脚等高级功能。该组件包含多个子组件：`SidebarMenu`（菜单）、`SidebarMenuItem`（菜单项）、`SidebarHeader`（页眉）和 `SidebarFooter`（页脚）。Sidebar特别适合应用程序的导航结构和应用程序布局。

## API 定义

### Sidebar 结构

```rust
pub struct Sidebar<E: Collapsible + IntoElement + 'static> {
    style: StyleRefinement,
    content: Vec<E>,
    header: Option<AnyElement>,
    footer: Option<AnyElement>,
    side: Side,
    collapsible: bool,
    collapsed: bool,
}
```

### SidebarMenu 结构

```rust
pub struct SidebarMenu {
    style: StyleRefinement,
    collapsed: bool,
    items: Vec<SidebarMenuItem>,
}
```

### SidebarMenuItem 结构

```rust
pub struct SidebarMenuItem {
    id: ElementId,
    icon: Option<Icon>,
    label: SharedString,
    handler: Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>,
    active: bool,
    default_open: bool,
    click_to_open: bool,
    collapsed: bool,
    children: Vec<Self>,
    suffix: Option<AnyElement>,
    disabled: bool,
}
```

### SidebarHeader 结构

```rust
pub struct SidebarHeader {
    base: Div,
    style: StyleRefinement,
    children: Vec<AnyElement>,
    selected: bool,
    collapsed: bool,
}
```

### SidebarFooter 结构

```rust
pub struct SidebarFooter {
    base: Div,
    selected: bool,
    collapsed: bool,
}
```

### SidebarToggleButton 结构

```rust
pub struct SidebarToggleButton {
    btn: Button,
    collapsed: bool,
    side: Side,
    on_click: Option<Rc<dyn Fn(&ClickEvent, &mut Window, &mut App)>>,
}
```

## 主要方法

### Sidebar 方法

| 方法 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `new()` | `side: Side` | `Self` | 创建新的Sidebar，指定左右位置 |
| `left()` | - | `Self` | 创建左侧Sidebar |
| `right()` | - | `Self` | 创建右侧Sidebar |
| `collapsible()` | `collapsible: bool` | `Self` | 设置是否可收缩，默认true |
| `collapsed()` | `collapsed: bool` | `Self` | 设置初始状态为收缩 |
| `header()` | `header: impl IntoElement` | `Self` | 设置页眉 |
| `footer()` | `footer: impl IntoElement` | `Self` | 设置页脚 |
| `child()` | `child: E` | `Self` | 添加单个子元素 |
| `children()` | `children: impl IntoIterator<Item = E>` | `Self` | 添加多个子元素 |

### SidebarMenu 方法

| 方法 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `new()` | - | `Self` | 创建新的SidebarMenu |
| `child()` | `child: impl Into<SidebarMenuItem>` | `Self` | 添加菜单项 |
| `children()` | `children: impl IntoIterator<...>` | `Self` | 添加多个菜单项 |

### SidebarMenuItem 方法

| 方法 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `new()` | `label: impl Into<SharedString>` | `Self` | 创建新的菜单项 |
| `icon()` | `icon: impl Into<Icon>` | `Self` | 设置菜单项图标 |
| `active()` | `active: bool` | `Self` | 设置菜单项激活状态 |
| `on_click()` | `handler: impl Fn(...) + 'static` | `Self` | 添加点击处理器 |
| `collapsed()` | `collapsed: bool` | `Self` | 设置是否显示为收缩状态 |
| `default_open()` | `open: bool` | `Self` | 设置子菜单初始打开状态 |
| `click_to_open()` | `click_to_open: bool` | `Self` | 设置点击菜项时打开子菜单，默认false |
| `children()` | `children: impl IntoIterator<...>` | `Self` | 添加子菜单项 |
| `suffix()` | `suffix: impl IntoElement` | `Self` | 设置菜单项后缀元素 |
| `disable()` | `disable: bool` | `Self` | 禁用菜单项 |

### SidebarHeader 方法

| 方法 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `new()` | - | `Self` | 创建新的SidebarHeader |
| `selected()` | `selected: bool` | `Self` | 设置选中状态 |
| `collapsed()` | `collapsed: bool` | `Self` | 设置收缩状态 |
| `child()` / `extend()` | `elements: impl IntoIterator<...>` | - | 添加子元素 |

### SidebarFooter 方法

| 方法 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `new()` | - | `Self` | 创建新的SidebarFooter |
| `selected()` | `selected: bool` | `Self` | 设置选中状态 |
| `collapsed()` | `collapsed: bool` | `Self` | 设置收缩状态 |
| `extend()` | `elements: impl IntoIterator<...>` | - | 添加子元素 |

### SidebarToggleButton 方法

| 方法 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `left()` | - | `Self` | 创建左侧切换按钮 |
| `right()` | - | `Self` | 创建右侧切换按钮 |
| `side()` | `side: Side` | `Self` | 设置按钮位置 |
| `collapsed()` | `collapsed: bool` | `Self` | 设置当前收缩状态 |
| `on_click()` | `on_click: impl Fn(...) + 'static` | `Self` | 添加点击处理器 |

## 常量

| 常量 | 值 | 说明 |
|------|-----|------|
| `DEFAULT_WIDTH` | `px(255.)` | Sidebar默认宽度 |
| `COLLAPSED_WIDTH` | `px(48.)` | Sidebar收缩时宽度 |

## Side 枚举

```rust
pub enum Side {
    Left,    // 左侧
    Right,   // 右侧
}

impl Side {
    pub fn is_left(&self) -> bool;
    pub fn is_right(&self) -> bool;
}
```

## 使用示例

### 基础用法

```rust
use gpui::{prelude::*, cx};
use gpui_component::{Sidebar, SidebarMenu, SidebarMenuItem};

let sidebar = Sidebar::left()
    .child(
        SidebarMenu::new()
            .child(SidebarMenuItem::new("Home"))
            .child(SidebarMenuItem::new("Settings"))
    );

div().child(sidebar)
```

### 带页眉的Sidebar

```rust
Sidebar::left()
    .header(
        h_flex()
            .gap_2()
            .child(Icon::new(IconName::AppIcon))
            .child("My App")
    )
    .child(
        SidebarMenu::new()
            .child(SidebarMenuItem::new("Home"))
            .child(SidebarMenuItem::new("Dashboard"))
    )
```

### 带页脚的Sidebar

```rust
Sidebar::left()
    .footer(
        h_flex()
            .gap_2()
            .child(Icon::new(IconName::Settings))
            .child("Settings")
    )
    .child(
        SidebarMenu::new()
            .child(SidebarMenuItem::new("Home"))
    )
```

### 带菜单项的Sidebar

```rust
Sidebar::left()
    .child(
        SidebarMenu::new()
            .child(
                SidebarMenuItem::new("Dashboard")
                    .icon(Icon::new(IconName::LayoutDashboard))
                    .active(true)
            )
            .child(
                SidebarMenuItem::new("Projects")
                    .icon(Icon::new(IconName::Folder))
            )
            .child(
                SidebarMenuItem::new("Settings")
                    .icon(Icon::new(IconName::Settings))
            )
    )
```

### 菜单项点击事件

```rust
let sidebar = Sidebar::left()
    .child(
        SidebarMenu::new()
            .child(
                SidebarMenuItem::new("Home")
                    .icon(Icon::new(IconName::Home))
                    .on_click(|_, window, cx| {
                        println!("Home clicked");
                    })
            )
            .child(
                SidebarMenuItem::new("Settings")
                    .icon(Icon::new(IconName::Settings))
                    .on_click(|_, window, cx| {
                        println!("Settings clicked");
                    })
            )
    );
```

### 带子菜单的菜单项

```rust
Sidebar::left()
    .child(
        SidebarMenu::new()
            .child(
                SidebarMenuItem::new("Tools")
                    .icon(Icon::new(IconName::Wrench))
                    .children(vec![
                        SidebarMenuItem::new("Tool 1"),
                        SidebarMenuItem::new("Tool 2"),
                    ])
            )
    )
```

### 子菜单点击打开

```rust
SidebarMenuItem::new("Tools")
    .icon(Icon::new(IconName::Wrench))
    .click_to_open(true)  // 点击菜项时打开子菜单
    .default_open(false)  // 初始不打开
    .children(vec![
        SidebarMenuItem::new("Tool 1"),
        SidebarMenuItem::new("Tool 2"),
    ])
```

### 菜单项后缀

```rust
SidebarMenuItem::new("Downloads")
    .icon(Icon::new(IconName::Download))
    .suffix(
        Badge::new("5")  // 显示5条未读项
    )
```

### 禁用菜单项

```rust
SidebarMenu::new()
    .child(
        SidebarMenuItem::new("Beta Feature")
            .icon(Icon::new(IconName::Zap))
            .disable(true)  // 禁用此菜单项
    )
```

### 可收缩Sidebar

```rust
let mut sidebar_collapsed = false;

div()
    .flex()
    .child(
        Sidebar::left()
            .collapsible(true)  // 允许收缩
            .collapsed(sidebar_collapsed)
            .child(
                SidebarMenu::new()
                    .child(SidebarMenuItem::new("Home"))
            )
    )
    .child(
        SidebarToggleButton::left()
            .collapsed(sidebar_collapsed)
            .on_click(|_, _, cx| {
                sidebar_collapsed = !sidebar_collapsed;
            })
    )
```

### 右侧Sidebar

```rust
Sidebar::right()
    .header(
        Label::new("Properties")
    )
    .child(
        SidebarMenu::new()
            .child(SidebarMenuItem::new("General"))
            .child(SidebarMenuItem::new("Advanced"))
    )
```

### 完整的应用布局示例

```rust
h_flex()
    .size_full()
    .child(
        // 左侧导航
        Sidebar::left()
            .header(
                h_flex()
                    .gap_2()
                    .child(Icon::new(IconName::AppIcon))
                    .child(Label::new("MyApp"))
            )
            .child(
                SidebarMenu::new()
                    .child(
                        SidebarMenuItem::new("Dashboard")
                            .icon(Icon::new(IconName::LayoutDashboard))
                            .active(true)
                            .on_click(|_, _, cx| {})
                    )
                    .child(
                        SidebarMenuItem::new("Projects")
                            .icon(Icon::new(IconName::Folder))
                            .children(vec![
                                SidebarMenuItem::new("Recent"),
                                SidebarMenuItem::new("All Projects"),
                            ])
                    )
            )
            .footer(
                h_flex()
                    .gap_2()
                    .child(Icon::new(IconName::User))
                    .child(Label::new("Profile"))
            )
    )
    .child(
        // 主内容区域
        v_flex()
            .flex_1()
            .child(Label::new("Main Content"))
    )
    .child(
        // 右侧属性面板
        Sidebar::right()
            .header(Label::new("Properties"))
            .child(
                SidebarMenu::new()
                    .child(SidebarMenuItem::new("General"))
                    .child(SidebarMenuItem::new("Advanced"))
            )
    )
```

### 动态更新菜单项状态

```rust
// 通过状态管理改变菜单项的active状态
let active_menu = RefCell::new("home".to_string());

let sidebar = Sidebar::left()
    .child(
        SidebarMenu::new()
            .child(
                SidebarMenuItem::new("Home")
                    .active(active_menu.borrow().as_str() == "home")
                    .on_click(|_, _, cx| {
                        *active_menu.borrow_mut() = "home".to_string();
                    })
            )
            .child(
                SidebarMenuItem::new("Settings")
                    .active(active_menu.borrow().as_str() == "settings")
                    .on_click(|_, _, cx| {
                        *active_menu.borrow_mut() = "settings".to_string();
                    })
            )
    );
```

## Collapsible Trait

Sidebar的子元素必须实现 `Collapsible` trait：

```rust
pub trait Collapsible: Sized {
    fn is_collapsed(&self) -> bool;
    fn collapsed(self, collapsed: bool) -> Self;
}
```

这允许Sidebar在收缩状态下改变子元素的外观。

## 样式定制

Sidebar 实现了 `Styled` trait，支持GPUI的所有样式方法：

```rust
Sidebar::left()
    .p_4()
    .bg(cx.theme().sidebar)
    .text_color(cx.theme().sidebar_foreground)
```

## 尺寸说明

| 状态 | 宽度 |
|------|------|
| 展开 | 255px |
| 收缩 | 48px |

## 注意事项

1. **Collapsible Trait** - Sidebar的子元素必须实现 `Collapsible` trait，以支持收缩功能

2. **Side 参数** - 使用 `Side::Left` 或 `Side::Right` 控制Sidebar的位置，左侧显示右边框，右侧显示左边框

3. **菜单项递归** - SidebarMenuItem 可以无限递归添加子菜单，但建议不超过3级

4. **Icon 需要** - 推荐为菜单项添加图标，在收缩状态下仅显示图标

5. **活跃状态** - `active()` 用于高亮当前选中的菜单项，通常配合应用的路由使用

6. **禁用状态** - 禁用菜单项会改变其外观（灰显）并禁止点击事件

7. **Header/Footer** - 页眉和页脚在收缩状态下会自动调整内边距以适应更窄的宽度

8. **默认可收缩** - Sidebar默认 `collapsible: true`，可通过 `collapsible(false)` 禁用

## 相关组件

- [Menu](menu.md) - 通用菜单组件
- [Icon](icon.md) - 图标组件
- [Button](button.md) - 按钮组件，用于SidebarToggleButton
