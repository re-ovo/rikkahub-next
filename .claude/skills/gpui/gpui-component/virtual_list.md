# VirtualList 组件

## 概述

`VirtualList` 是一个高性能的虚拟列表组件，用于渲染大量项目而仅在可见范围内进行DOM更新。与GPUI的 `uniform_list` 不同，VirtualList支持不同高度/宽度的项目，可用于表格、列表等复杂场景。该组件支持垂直和水平两种方向，并提供了灵活的滚动控制接口。

## API 定义

### VirtualList 结构

```rust
pub struct VirtualList {
    id: ElementId,
    axis: Axis,
    base: Stateful<Div>,
    scroll_handle: VirtualListScrollHandle,
    items_count: usize,
    item_sizes: Rc<Vec<Size<Pixels>>>,
    render_items: Box<
        dyn for<'a> Fn(Range<usize>, &'a mut Window, &'a mut App) -> SmallVec<[AnyElement; 64]>,
    >,
    sizing_behavior: ListSizingBehavior,
}
```

### VirtualListScrollHandle 结构

```rust
pub struct VirtualListScrollHandle {
    state: Rc<RefCell<VirtualListScrollHandleState>>,
    base_handle: ScrollHandle,
}

struct VirtualListScrollHandleState {
    axis: Axis,
    items_count: usize,
    pub deferred_scroll_to_item: Option<DeferredScrollToItem>,
}
```

### VirtualListFrameState 结构

```rust
pub struct VirtualListFrameState {
    items: SmallVec<[AnyElement; 32]>,
    size_layout: ItemSizeLayout,
}

#[derive(Default, Clone)]
pub struct ItemSizeLayout {
    items_sizes: Rc<Vec<Size<Pixels>>>,
    content_size: Size<Pixels>,
    sizes: Vec<Pixels>,
    origins: Vec<Pixels>,
    last_layout_bounds: Bounds<Pixels>,
}
```

## 主要函数和方法

### 创建函数

| 函数 | 参数 | 说明 |
|------|------|------|
| `v_virtual_list()` | `view, id, item_sizes, f` | 创建垂直虚拟列表 |
| `h_virtual_list()` | `view, id, item_sizes, f` | 创建水平虚拟列表 |

### VirtualList 方法

| 方法 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `track_scroll()` | `scroll_handle: &VirtualListScrollHandle` | `Self` | 跟踪滚动事件 |
| `with_sizing_behavior()` | `behavior: ListSizingBehavior` | `Self` | 设置尺寸行为（Infer/Auto） |
| `with_scroll_handle()` | `scroll_handle: &VirtualListScrollHandle` | `Self` | 指定滚动句柄（仅用于Table） |

### VirtualListScrollHandle 方法

| 方法 | 参数 | 返回值 | 说明 |
|------|------|--------|------|
| `new()` | - | `Self` | 创建新的滚动句柄 |
| `scroll_to_item()` | `ix: usize, strategy: ScrollStrategy` | - | 滚动到指定项目 |
| `scroll_to_bottom()` | - | - | 滚动到列表底部 |
| `base_handle()` | - | `&ScrollHandle` | 获取基础滚动句柄 |
| `offset()` | - | `Point<Pixels>` | 获取当前滚动偏移 |
| `set_offset()` | `offset: Point<Pixels>` | - | 设置滚动偏移 |
| `content_size()` | - | `Size<Pixels>` | 获取内容总大小 |

## ListSizingBehavior 枚举

```rust
pub enum ListSizingBehavior {
    Infer,  // 推断尺寸（根据内容自动调整）
    Auto,   // 自动尺寸（适应容器）
}
```

## ScrollStrategy 枚举

```rust
pub enum ScrollStrategy {
    Top,      // 滚动到项目顶部
    Center,   // 滚动到项目中心
    Bottom,   // 滚动到项目底部（向下滚动时使用）
    Nearest,  // 滚动到最近的可见位置
}
```

## 使用示例

### 基础垂直列表

```rust
use gpui::{prelude::*, cx, Pixels, px};
use gpui_component::virtual_list::{v_virtual_list, VirtualListScrollHandle};

let scroll_handle = VirtualListScrollHandle::new();

// 创建项目尺寸数组
let item_sizes = std::rc::Rc::new(
    (0..1000)
        .map(|_| gpui::size(px(300.), px(50.)))
        .collect::<Vec<_>>()
);

let view = cx.entity();

v_virtual_list(
    view,
    "list",
    item_sizes.clone(),
    |_, range, _, _| {
        range.map(|i| {
            div()
                .h_12()
                .flex()
                .items_center()
                .child(format!("Item {}", i))
        }).collect()
    }
)
.track_scroll(&scroll_handle)
```

### 基础水平列表

```rust
let scroll_handle = VirtualListScrollHandle::new();

let item_sizes = std::rc::Rc::new(
    (0..500)
        .map(|_| gpui::size(px(120.), px(100.)))
        .collect::<Vec<_>>()
);

h_virtual_list(
    view,
    "horizontal_list",
    item_sizes.clone(),
    |_, range, _, _| {
        range.map(|i| {
            div()
                .w_30()
                .h_24()
                .flex()
                .items_center()
                .justify_center()
                .child(format!("Col {}", i))
        }).collect()
    }
)
.track_scroll(&scroll_handle)
```

### 不同高度的项目

```rust
let scroll_handle = VirtualListScrollHandle::new();

// 每个项目有不同的高度
let item_sizes = std::rc::Rc::new(vec![
    gpui::size(px(400.), px(50.)),   // 第一项：50px高
    gpui::size(px(400.), px(80.)),   // 第二项：80px高
    gpui::size(px(400.), px(100.)),  // 第三项：100px高
    gpui::size(px(400.), px(60.)),   // 重复...
    gpui::size(px(400.), px(70.)),
    // ... 更多项
]);

v_virtual_list(
    view,
    "variable_height_list",
    item_sizes.clone(),
    |_, range, _, _| {
        range.map(|i| {
            div()
                .child(format!("Item {}", i))
        }).collect()
    }
)
.track_scroll(&scroll_handle)
```

### 滚动到指定项目

```rust
let scroll_handle = VirtualListScrollHandle::new();

// 创建虚拟列表...
let list = v_virtual_list(/* ... */);

// 点击按钮时滚动到第10项
Button::new("scroll-to-10")
    .on_click({
        let handle = scroll_handle.clone();
        move |_, _, _| {
            handle.scroll_to_item(10, ScrollStrategy::Center);
        }
    })
```

### 滚动到底部

```rust
let scroll_handle = VirtualListScrollHandle::new();

// 创建虚拟列表...

// 滚动到底部
Button::new("scroll-bottom")
    .on_click({
        let handle = scroll_handle.clone();
        move |_, _, _| {
            handle.scroll_to_bottom();
        }
    })
```

### 完整的列表示例

```rust
use gpui::{prelude::*, cx};
use gpui_component::virtual_list::{v_virtual_list, VirtualListScrollHandle};

let scroll_handle = VirtualListScrollHandle::new();

// 假设这是从数据源获取的项目列表
let items = vec!["Apple", "Banana", "Cherry", "Date", /* ... */];

let item_sizes = std::rc::Rc::new(
    items.iter().map(|_| gpui::size(px(400.), px(48.))).collect::<Vec<_>>()
);

v_flex()
    .flex_1()
    .gap_2()
    .child(
        h_flex()
            .gap_2()
            .child(
                Button::new("jump-top")
                    .label("Go to Top")
                    .on_click({
                        let handle = scroll_handle.clone();
                        move |_, _, _| {
                            handle.scroll_to_item(0, ScrollStrategy::Top);
                        }
                    })
            )
            .child(
                Button::new("jump-middle")
                    .label("Go to Middle")
                    .on_click({
                        let handle = scroll_handle.clone();
                        move |_, _, _| {
                            handle.scroll_to_item(
                                items.len() / 2,
                                ScrollStrategy::Center,
                            );
                        }
                    })
            )
            .child(
                Button::new("jump-bottom")
                    .label("Go to Bottom")
                    .on_click({
                        let handle = scroll_handle.clone();
                        move |_, _, _| {
                            handle.scroll_to_bottom();
                        }
                    })
            )
    )
    .child(
        v_virtual_list(
            view,
            "main-list",
            item_sizes,
            |_, range, _, _| {
                range.map(|i| {
                    div()
                        .h_12()
                        .px_4()
                        .flex()
                        .items_center()
                        .bg(if i % 2 == 0 {
                            cx.theme().background
                        } else {
                            cx.theme().muted
                        })
                        .child(format!("Item {}: {}", i, items[i]))
                }).collect()
            }
        )
        .track_scroll(&scroll_handle)
    )
```

### 表格行虚拟化

```rust
let scroll_handle = VirtualListScrollHandle::new();

// 表格行高度
let row_height = px(40.);
let rows_count = 10000;

let row_sizes = std::rc::Rc::new(
    (0..rows_count)
        .map(|_| gpui::size(px(800.), row_height))
        .collect::<Vec<_>>()
);

v_virtual_list(
    view,
    "table-body",
    row_sizes,
    |_, range, _, _| {
        range.map(|i| {
            h_flex()
                .w_full()
                .h_10()
                .px_4()
                .gap_4()
                .items_center()
                .border_b_1()
                .child(format!("Row {}", i))
                .child(format!("Data {}", i))
        }).collect()
    }
)
.track_scroll(&scroll_handle)
```

### 带搜索过滤的列表

```rust
struct ListView {
    filtered_items: Vec<String>,
    scroll_handle: VirtualListScrollHandle,
}

impl ListView {
    fn filter_items(&mut self, query: &str) {
        self.filtered_items = self.all_items
            .iter()
            .filter(|item| item.to_lowercase().contains(&query.to_lowercase()))
            .cloned()
            .collect();

        // 重置滚动位置
        self.scroll_handle.scroll_to_item(0, ScrollStrategy::Top);
    }
}

// 使用时
let mut view = ListView { /* ... */ };

Input::new(&search_input)
    .on_change({
        let mut view = view.clone();
        move |text, _, _| {
            view.filter_items(&text);
        }
    })
```

### 无限滚动列表

```rust
let scroll_handle = VirtualListScrollHandle::new();
let mut loaded_items = 100;
let mut loading = false;

v_virtual_list(
    view,
    "infinite-list",
    item_sizes.clone(),
    |_, range, window, cx| {
        // 检查是否接近底部
        if range.end > loaded_items - 20 && !loading {
            loading = true;
            // 发送请求加载更多项
            load_more_items(window, cx);
        }

        range.map(|i| {
            div()
                .h_12()
                .child(format!("Item {}", i))
        }).collect()
    }
)
.track_scroll(&scroll_handle)
```

## 性能注意事项

1. **项目尺寸数组** - `item_sizes` 的长度必须与实际项目数量相同，否则会导致渲染错误

2. **虚拟化范围** - 仅渲染可见项目加上一定的缓冲区，可显著提高大列表的性能

3. **项目高度/宽度** - 对于垂直列表，item_sizes 中的高度很重要；对于水平列表，宽度很重要

4. **渲染函数** - `render_items` 闭包应该是轻量级的，避免复杂的计算

5. **滚动策略** - 不同的 `ScrollStrategy` 会影响滚动行为，选择合适的策略可提高用户体验

## 注意事项

1. **必需的Entity** - VirtualList 需要一个Render view的Entity来创建（通过cx.entity()）

2. **Item Sizes 大小** - `item_sizes` 向量的长度必须与要渲染的项目数量相匹配

3. **性能限制** - 虽然VirtualList很高效，但数千项目仍可能需要优化

4. **滚动同步** - 如果有多个VirtualList需要同步滚动，需要共享同一个 `VirtualListScrollHandle`

5. **方向转换** - 创建后无法改变列表方向（垂直/水平），需要重新创建

6. **动态项目** - 如果项目高度/宽度需要动态改变，需要重新创建项目尺寸数组

7. **嵌套列表** - 嵌套VirtualList可能导致性能问题，应谨慎使用

8. **焦点管理** - VirtualList 不自动管理焦点，需要在应用层处理

## Axis 枚举

```rust
pub enum Axis {
    Vertical,    // 垂直滚动
    Horizontal,  // 水平滚动
}
```

## 与 uniform_list 的区别

| 特性 | VirtualList | uniform_list |
|------|-------------|--------------|
| 项目大小 | 可变 | 统一 |
| 性能 | 高（仅渲染可见项）| 最高（针对统一大小优化） |
| 使用复杂度 | 中等 | 低 |
| 适用场景 | 表格、混合高度列表 | 统一项目列表 |

## 相关组件

- [List](list.md) - 通用列表组件，适用于少量项目
- [Table](table.md) - 表格组件，可使用VirtualList优化大数据表格
