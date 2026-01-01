# Select 组件

## 概述

`Select` 是一个功能完整的下拉选择组件，支持单选、搜索、分组、自定义渲染等高级功能。通过 `SelectDelegate` trait 实现了灵活的数据源管理，支持矢量数据、分组数据等。组件采用状态管理模式，所有交互通过 `SelectState` 进行管理。

## API 定义

### Select 结构

```rust
pub struct Select<D: SelectDelegate + 'static> {
    id: ElementId,
    state: Entity<SelectState<D>>,
    options: SelectOptions,
}

struct SelectOptions {
    style: StyleRefinement,
    size: Size,
    icon: Option<Icon>,
    cleanable: bool,
    placeholder: Option<SharedString>,
    title_prefix: Option<SharedString>,
    search_placeholder: Option<SharedString>,
    empty: Option<AnyElement>,
    menu_width: Length,
    disabled: bool,
    appearance: bool,
}
```

### SelectState 结构

```rust
pub struct SelectState<D: SelectDelegate + 'static> {
    focus_handle: FocusHandle,
    options: SelectOptions,
    searchable: bool,
    list: Entity<ListState<SelectListDelegate<D>>>,
    empty: Option<Box<dyn Fn(&Window, &App) -> AnyElement>>,
    bounds: Bounds<Pixels>,
    open: bool,
    selected_value: Option<<D::Item as SelectItem>::Value>,
    final_selected_index: Option<IndexPath>,
    _subscriptions: Vec<Subscription>,
}
```

### SelectItem Trait

```rust
pub trait SelectItem: Clone {
    type Value: Clone;
    fn title(&self) -> SharedString;
    fn display_title(&self) -> Option<AnyElement> {
        None
    }
    fn render(&self, _: &mut Window, _: &mut App) -> impl IntoElement {
        self.title().into_element()
    }
    fn value(&self) -> &Self::Value;
    fn matches(&self, query: &str) -> bool {
        self.title().to_lowercase().contains(&query.to_lowercase())
    }
}
```

### SelectDelegate Trait

```rust
pub trait SelectDelegate: Sized {
    type Item: SelectItem;

    fn sections_count(&self, _: &App) -> usize {
        1
    }

    fn section(&self, _section: usize) -> Option<AnyElement> {
        None
    }

    fn items_count(&self, section: usize) -> usize;
    fn item(&self, ix: IndexPath) -> Option<&Self::Item>;

    fn position<V>(&self, _value: &V) -> Option<IndexPath>
    where
        Self::Item: SelectItem<Value = V>,
        V: PartialEq;

    fn perform_search(
        &mut self,
        _query: &str,
        _window: &mut Window,
        _: &mut Context<SelectState<Self>>,
    ) -> Task<()> {
        Task::ready(())
    }
}
```

## 主要方法

### Select 方法

| 方法 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `new()` | `state: &Entity<SelectState<D>>` | `Self` | 创建新的Select实例 |
| `with_size()` | `size: impl Into<Size>` | `Self` | 设置下拉框大小 |
| `placeholder()` | `placeholder: impl Into<SharedString>` | `Self` | 设置占位符文本 |
| `icon()` | `icon: impl Into<Icon>` | `Self` | 设置自定义下拉箭头图标 |
| `title_prefix()` | `prefix: impl Into<SharedString>` | `Self` | 设置选中项的前缀 |
| `cleanable()` | `cleanable: bool` | `Self` | 启用清除按钮 |
| `search_placeholder()` | `placeholder: impl Into<SharedString>` | `Self` | 设置搜索框占位符 |
| `disabled()` | `disabled: bool` | `Self` | 禁用下拉框 |
| `empty()` | `el: impl IntoElement` | `Self` | 设置空列表时的显示元素 |
| `appearance()` | `appearance: bool` | `Self` | 控制外观样式 |
| `menu_width()` | `width: impl Into<Length>` | `Self` | 设置下拉菜单宽度 |

### SelectState 方法

| 方法 | 说明 |
|------|------|
| `new()` | 创建新的SelectState，需要在GPUI上下文中调用 |
| `searchable()` | 启用搜索功能 |
| `set_selected_index()` | 按索引设置选中项 |
| `set_selected_value()` | 按值设置选中项 |
| `set_items()` | 设置新的数据源 |
| `selected_index()` | 获取当前选中项的索引 |
| `selected_value()` | 获取当前选中项的值 |
| `focus()` | 获取焦点 |

## 内置实现

### SearchableVec

```rust
pub struct SearchableVec<T> {
    items: Vec<T>,
    matched_items: Vec<T>,
}

impl<T: Clone> SearchableVec<T> {
    pub fn new(items: impl Into<Vec<T>>) -> Self;
    pub fn push(&mut self, item: T);
}

impl<T: SelectItem> SelectDelegate for SearchableVec<T>;
```

SearchableVec 为Vec类型自动实现SelectDelegate，支持搜索功能。

### SelectGroup

```rust
pub struct SelectGroup<I: SelectItem> {
    pub title: SharedString,
    pub items: Vec<I>,
}

impl<I: SelectItem> SelectGroup<I> {
    pub fn new(title: impl Into<SharedString>) -> Self;
    pub fn item(mut self, item: I) -> Self;
    pub fn items(mut self, items: impl IntoIterator<Item = I>) -> Self;
}
```

SelectGroup 用于创建分组的选项，与 SearchableVec<SelectGroup<T>> 配合使用。

## 使用示例

### 基础用法

```rust
use gpui::{prelude::*, cx};
use gpui_component::select::{Select, SelectState, SearchableVec};

let select_state = cx.new(|cx| {
    let delegate = SearchableVec::new(vec![
        "Option 1",
        "Option 2",
        "Option 3",
    ]);
    SelectState::new(delegate, None, window, cx)
});

div().child(
    Select::new(&select_state)
        .with_size(Size::Medium)
        .placeholder("Select an option")
)
```

### 带搜索功能

```rust
let select_state = cx.new(|cx| {
    let delegate = SearchableVec::new(vec![
        "Apple",
        "Banana",
        "Cherry",
        "Date",
    ]);
    SelectState::new(delegate, None, window, cx)
});

Select::new(&select_state)
    .with_size(Size::Medium)
    .placeholder("Choose a fruit")
    // 启用搜索
    .when(/* condition */, |this| {
        select_state.update(cx, |state, _| {
            *state = state.clone().searchable(true);
        });
        this
    })
```

### 分组选项

```rust
let items = vec![
    SelectGroup::new("Fruits")
        .items(vec!["Apple", "Banana", "Cherry"]),
    SelectGroup::new("Vegetables")
        .items(vec!["Carrot", "Lettuce", "Tomato"]),
];

let select_state = cx.new(|cx| {
    let delegate = SearchableVec::new(items);
    SelectState::new(delegate, None, window, cx)
});

Select::new(&select_state)
    .with_size(Size::Medium)
    .placeholder("Choose a food")
```

### 清除按钮

```rust
Select::new(&select_state)
    .with_size(Size::Medium)
    .placeholder("Select...")
    .cleanable(true)  // 显示清除按钮
```

### 自定义图标

```rust
use gpui_component::Icon;

Select::new(&select_state)
    .with_size(Size::Medium)
    .placeholder("Select...")
    .icon(Icon::new(IconName::Settings))  // 自定义图标替代默认箭头
```

### 标题前缀

```rust
Select::new(&select_state)
    .with_size(Size::Medium)
    .placeholder("Select country")
    .title_prefix("Country: ")  // 显示为 "Country: United States"
```

### 空状态显示

```rust
Select::new(&select_state)
    .with_size(Size::Medium)
    .placeholder("Select...")
    .empty(
        h_flex()
            .justify_center()
            .py_6()
            .text_color(cx.theme().muted_foreground)
            .child("No options available")
    )
```

### 禁用状态

```rust
Select::new(&select_state)
    .with_size(Size::Medium)
    .disabled(true)
    .placeholder("Disabled")
```

### 无边框样式

```rust
Select::new(&select_state)
    .with_size(Size::Small)
    .appearance(false)  // 无背景和边框
```

### 完整的表单示例

```rust
v_flex()
    .gap_3()
    .child(
        h_flex()
            .gap_2()
            .child(Label::new("Country:"))
            .child(
                Select::new(&country_state)
                    .with_size(Size::Medium)
                    .placeholder("Choose a country")
                    .cleanable(true)
            )
    )
    .child(
        h_flex()
            .gap_2()
            .child(Label::new("Category:"))
            .child(
                Select::new(&category_state)
                    .with_size(Size::Medium)
                    .placeholder("Choose a category")
                    .search_placeholder("Search categories...")
            )
    )
```

### 响应选择事件

```rust
// 在SelectState创建时监听选择事件
let state = cx.new(|cx| {
    let delegate = SearchableVec::new(items);
    let mut state = SelectState::new(delegate, None, window, cx);

    // SelectState 发出 SelectEvent::Confirm 事件
    cx.subscribe(&state, |_, event, _, _| {
        match event {
            SelectEvent::Confirm(Some(value)) => {
                println!("Selected: {:?}", value);
            }
            SelectEvent::Confirm(None) => {
                println!("Selection cleared");
            }
        }
    }).detach();

    state
});
```

## 自定义 SelectDelegate 实现

```rust
use gpui_component::select::{SelectDelegate, SelectItem, IndexPath};

struct Country {
    name: String,
    code: String,
}

impl SelectItem for Country {
    type Value = String;

    fn title(&self) -> SharedString {
        self.name.clone().into()
    }

    fn value(&self) -> &Self::Value {
        &self.code
    }
}

struct CountryDelegate {
    items: Vec<Country>,
}

impl SelectDelegate for CountryDelegate {
    type Item = Country;

    fn items_count(&self, _: usize) -> usize {
        self.items.len()
    }

    fn item(&self, ix: IndexPath) -> Option<&Self::Item> {
        self.items.get(ix.row)
    }

    fn position<V>(&self, value: &V) -> Option<IndexPath>
    where
        Self::Item: SelectItem<Value = V>,
        V: PartialEq,
    {
        self.items.iter().position(|item| item.value() == value)
            .map(|ix| IndexPath::default().row(ix))
    }
}
```

## 事件

| 事件 | 说明 |
|------|------|
| `SelectEvent::Confirm(Option<V>)` | 用户确认选择时触发，包含选中的值 |
| `DismissEvent` | 下拉菜单关闭时触发 |

## Size 尺寸

| 尺寸 | 说明 |
|------|------|
| `Size::Small` | 小尺寸 |
| `Size::Medium` | 中等尺寸（默认） |
| `Size::Large` | 大尺寸 |

## 键盘快捷键

| 快捷键 | 操作 |
|--------|------|
| Up Arrow | 上一个选项 |
| Down Arrow | 下一个选项 |
| Enter | 确认选择 |
| Escape | 关闭下拉菜单 |

## 注意事项

1. **SelectState 必须在GPUI上下文中创建** - 调用 `SelectState::new()` 需要在 `cx.new()` 或 `cx.update()` 中进行

2. **搜索功能** - 搜索需要通过 `searchable()` 方法启用，并且委托必须实现 `perform_search()` 方法

3. **值类型** - SelectItem的Value类型必须实现 `Clone` 和 `PartialEq` traits

4. **分组搜索** - 使用分组时，搜索会递归过滤分组内的项目

5. **菜单宽度** - 默认为 `Length::Auto`，会与输入框宽度相同，可通过 `menu_width()` 自定义

6. **受控组件** - Select通过 `set_selected_value()` 可以实现受控组件模式

7. **空列表处理** - 当委托的项目数为0时，会显示 `empty()` 设置的元素或默认的空状态

8. **焦点管理** - 点击输入框时自动打开下拉菜单，按Escape或点击外部关闭

9. **搜索占位符** - `search_placeholder()` 仅在启用搜索时有效

## 相关组件

- [Input](input.md) - 输入框组件，可用于搜索功能
- [List](list.md) - 列表组件，Select内部使用
- [Icon](icon.md) - 图标组件，用于下拉箭头
