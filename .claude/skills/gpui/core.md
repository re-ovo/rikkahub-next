# GPUI 核心API

> 基于 GPUI 的深入理解和实践总结

## 目录

- [核心概念](#核心概念)
  - [AppContext - 统一接口](#appcontext---统一接口)
  - [App - 全局管理器](#app---全局管理器)
  - [Context<T> - 实体上下文](#contextt---实体上下文)
  - [Entity<T> - 智能指针](#entityt---智能指针)
  - [四者关系图](#四者关系图)
- [组件定义](#组件定义)
  - [Render Trait](#render-trait)
  - [基础组件示例](#基础组件示例)
- [状态管理](#状态管理)
  - [状态读取](#状态读取)
  - [状态更新](#状态更新)
  - [响应式更新](#响应式更新)
- [跨组件状态传递](#跨组件状态传递)
  - [方式1: Global 全局状态](#方式1-global-全局状态)
  - [方式2: Entity 引用传递](#方式2-entity-引用传递)
  - [方式3: observe 观察变化](#方式3-observe-观察变化)
  - [方式4: EventEmitter + subscribe](#方式4-eventemitter--subscribe)
  - [方式5: 父子双向通信](#方式5-父子双向通信)
- [完整实例](#完整实例)
- [最佳实践](#最佳实践)

---

## 核心概念

GPUI 的核心围绕着四个关键类型展开：`AppContext`、`App`、`Context<T>` 和 `Entity<T>`。

### AppContext - 统一接口

**AppContext** 是一个 **trait（特征）**，定义了与应用程序上下文交互的统一接口。

```rust
pub trait AppContext {
    type Result<T>;  // 用于支持异步上下文

    // 实体生命周期管理
    fn new<T>(&mut self, build: impl FnOnce(&mut Context<T>) -> T) -> Self::Result<Entity<T>>;
    fn update_entity<T, R>(&mut self, handle: &Entity<T>, update: ...) -> Self::Result<R>;
    fn read_entity<T, R>(&self, handle: &Entity<T>, read: ...) -> Self::Result<R>;

    // 窗口管理
    fn update_window<T, F>(&mut self, window: AnyWindowHandle, update: F) -> Result<T>;

    // 全局状态
    fn read_global<G, R>(&self, callback: ...) -> Self::Result<R> where G: Global;

    // 异步任务
    fn background_spawn<R>(&self, future: ...) -> Task<R>;
}
```

**主要实现者**：
- `App` - 同步全局上下文
- `Context<T>` - 同步实体上下文
- `AsyncApp` - 异步全局上下文（可跨 await）
- `AsyncWindowContext` - 异步窗口上下文

### App - 全局管理器

**App** 是 `AppContext` 的主要实现，管理整个应用程序的状态。

```rust
pub struct App {
    // 实体存储
    pub(crate) entities: EntityMap,

    // 窗口管理
    pub(crate) windows: SlotMap<WindowId, Option<Box<Window>>>,

    // 全局状态
    pub(crate) globals_by_type: FxHashMap<TypeId, Box<dyn Any>>,

    // 事件系统
    pub(crate) observers: SubscriberSet<EntityId, Handler>,
    pub(crate) event_listeners: SubscriberSet<EntityId, (TypeId, Listener)>,

    // 执行器
    pub(crate) background_executor: BackgroundExecutor,
    pub(crate) foreground_executor: ForegroundExecutor,

    // 平台抽象
    pub(crate) platform: Rc<dyn Platform>,

    // ... 更多字段
}
```

**职责**：
- 存储所有实体数据（在 `EntityMap` 中）
- 管理所有窗口
- 维护全局状态
- 处理事件分发
- 管理异步任务

### Context<T> - 实体上下文

**Context<T>** 是特定实体的操作上下文，提供响应式编程 API。

```rust
pub struct Context<'a, T> {
    app: &'a mut App,           // 持有 App 的可变引用
    entity_state: WeakEntity<T>, // 关联的实体
}

// Context<T> 可以解引用为 App
impl<T> Deref for Context<'_, T> {
    type Target = App;
    fn deref(&self) -> &Self::Target {
        self.app
    }
}

// Context<T> 也实现了 AppContext
impl<T> AppContext for Context<'_, T> {
    // 所有方法都委托给内部的 App
    fn new<U>(&mut self, build: ...) -> Entity<U> {
        self.app.new(build)  // 委托
    }
    // ...
}
```

**特有功能**（相比 App）：
- `cx.entity()` - 获取当前实体的句柄
- `cx.notify()` - 通知观察者当前实体已变更
- `cx.emit(event)` - 发射事件
- `cx.observe(&entity, callback)` - 观察其他实体
- `cx.subscribe(&entity, callback)` - 订阅事件
- `cx.spawn(future)` - 启动异步任务（携带当前实体上下文）

### Entity<T> - 智能指针

**Entity<T>** 是实体的**句柄（智能指针）**，**不持有数据本身**。

```rust
pub struct Entity<T> {
    any_entity: AnyEntity,
    entity_type: PhantomData<fn(T) -> T>,  // 零大小类型标记
}

pub struct AnyEntity {
    entity_id: EntityId,                    // 只是一个 ID
    entity_type: TypeId,                    // 类型信息
    entity_map: Weak<RwLock<RefCounts>>,   // 引用计数
}
```

**关键特点**：
- **不持有数据**：`Entity<T>` 大小约 32 字节，无论 `T` 多大
- **引用计数**：类似 `Rc<T>`，多个 Entity 可指向同一数据
- **需要 AppContext 访问数据**：必须通过 `entity.read(cx)` 或 `entity.update(cx)` 访问

```rust
let entity: Entity<Counter> = ...;

// ❌ 错误：Entity 不包含数据
// entity.count

// ✅ 正确：通过 AppContext 访问
entity.read(&app, |data, _| {
    println!("{}", data.count);  // data 才是 &Counter
});

entity.update(&mut app, |data, cx| {
    data.count += 1;  // data 是 &mut Counter
    cx.notify();
});
```

### 四者关系图

```
┌─────────────────────────────────────────────┐
│          AppContext (trait)                 │
│  定义：如何操作实体的统一接口                  │
└─────────────────────────────────────────────┘
              ▲          ▲
              │          │
              │          │ 实现
    ┌─────────┴──┐  ┌───┴─────────────┐
    │   App      │  │  Context<T>     │
    │  全局管理   │  │  实体专属上下文   │
    └─────────────┘  └─────────────────┘
         │                   │
         │                   │ 绑定当前实体
         │ 管理所有实体        │
         ▼                   ▼
    ┌─────────────────────────────────┐
    │    EntityMap                     │
    │  [id_1]: Box<Counter { ... }>   │
    │  [id_2]: Box<Window { ... }>    │
    │  [id_3]: Box<Timer { ... }>     │
    └─────────────────────────────────┘
         ▲
         │ 通过 ID 访问
         │
    ┌────┴──────────┐
    │ Entity<T>     │  ← 智能指针/句柄
    │ - entity_id   │
    │ - type marker │
    └───────────────┘
```

**关系总结**：
- **AppContext** = 接口协议
- **App** = 全局数据存储 + 管理
- **Context<T>** = App 包装器 + 实体绑定 + 响应式 API
- **Entity<T>** = 数据访问凭证（钥匙🔑）

**数据流**：
```
Entity<T> --[需要 AppContext]--> App.entities[id] --> 真实数据
```

---

## 组件定义

### Render Trait

在 GPUI 中，**组件 = struct + Render trait**。

```rust
pub trait Render: 'static + Sized {
    /// 渲染方法：返回 UI 树
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement;
}
```

**关键点**：
- `&mut self` - 可以直接访问和修改状态
- `cx: &mut Context<Self>` - 实体专属上下文
- 返回 `impl IntoElement` - UI 元素树

### 基础组件示例

#### 最简单的组件

```rust
use gpui::*;

struct HelloWorld {
    text: String,
}

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .bg(rgb(0x505050))
            .size_full()
            .justify_center()
            .items_center()
            .child(format!("Hello, {}!", self.text))
    }
}

// 使用
fn main() {
    Application::new().run(|app| {
        app.open_window(WindowOptions::default(), |window, cx| {
            cx.new(|_| HelloWorld {
                text: "GPUI".to_string(),
            })
        });
    });
}
```

#### 带状态的交互组件

```rust
struct Counter {
    count: i32,
}

impl Counter {
    // 状态修改方法
    fn increment(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.count += 1;
        cx.notify();  // ⭐ 关键：通知重新渲染
    }

    fn decrement(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.count -= 1;
        cx.notify();
    }
}

impl Render for Counter {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_4()
            // 显示当前值
            .child(
                div().child(format!("Count: {}", self.count))
            )
            // 按钮
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(
                        button("Increment")
                            .on_click(cx.listener(Self::increment))
                    )
                    .child(
                        button("Decrement")
                            .on_click(cx.listener(Self::decrement))
                    )
            )
    }
}
```

---

## 状态管理

### 状态读取

在 `render()` 方法中可以直接访问 `self`：

```rust
impl Render for Counter {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().child(format!("Count: {}", self.count))
        //                                ^^^^^^^^^^
        //                                直接访问状态
    }
}
```

### 状态更新

状态更新的**黄金法则**：

```
状态修改 → cx.notify() → 重新调用 render() → 更新 UI
```

```rust
impl Counter {
    fn increment(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.count += 1;  // 1. 修改状态
        cx.notify();      // 2. ⭐ 通知系统
    }
}
```

**⚠️ 常见错误**：

```rust
// ❌ 错误：忘记 notify
fn increment(&mut self, ..., cx: &mut Context<Self>) {
    self.count += 1;
    // 缺少 cx.notify()，UI 不会更新！
}

// ✅ 正确
fn increment(&mut self, ..., cx: &mut Context<Self>) {
    self.count += 1;
    cx.notify();  // UI 会重新渲染
}
```

### 响应式更新

#### cx.listener() - 事件绑定

`cx.listener()` 是连接 UI 事件和状态修改的桥梁：

```rust
button("Click me")
    .on_click(cx.listener(|this, event, window, cx| {
        // this: &mut Counter       - 当前组件实例
        // event: &ClickEvent       - 事件数据
        // window: &mut Window      - 窗口引用
        // cx: &mut Context<Counter> - 上下文

        this.count += 1;
        cx.notify();
    }))
```

**简化写法**（使用方法引用）：

```rust
button("Increment")
    .on_click(cx.listener(Self::increment))
    // 相当于：
    // .on_click(cx.listener(|this, event, window, cx| {
    //     this.increment(event, window, cx)
    // }))
```

#### 条件渲染

```rust
impl Render for Popover {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .child(
                button("Open")
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.open = true;
                        cx.notify();
                    }))
                    // 条件渲染
                    .when(self.open, |button| {
                        button.child("Popover content")
                    })
            )
    }
}
```

---

## 跨组件状态传递

GPUI 提供了 5 种跨组件传递状态的方式，适用于不同场景。

### 方式1: Global 全局状态

适用于**应用级配置**，类似 React 的 Context API。

#### 定义和使用

```rust
// 1. 定义全局状态
struct AppSettings {
    theme: String,
    language: String,
}

// 2. 实现 Global trait
impl Global for AppSettings {}

// 3. 初始化时设置
fn main() {
    Application::new().run(|cx: &mut App| {
        cx.set_global(AppSettings {
            theme: "dark".to_string(),
            language: "zh-CN".to_string(),
        });
    });
}

// 4. 任何组件都可以读取
impl Render for MyComponent {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let settings = cx.global::<AppSettings>();

        div()
            .child(format!("Current theme: {}", settings.theme))
    }
}

// 5. 更新全局状态
fn change_theme(cx: &mut Context<Self>) {
    cx.update_global::<AppSettings, _>(|settings, cx| {
        settings.theme = "light".to_string();
        // 通知所有观察者
        cx.notify_global::<AppSettings>();
    });
}

// 6. 观察全局状态变化
struct ThemeAwareComponent;

impl ThemeAwareComponent {
    fn new(cx: &mut Context<Self>) -> Self {
        cx.observe_global::<AppSettings>(|this, cx| {
            // AppSettings 变化时触发
            cx.notify();  // 重新渲染
        }).detach();

        Self
    }
}
```

**适用场景**：
- ✅ 应用主题
- ✅ 用户信息
- ✅ 全局配置
- ❌ 组件局部状态

### 方式2: Entity 引用传递

父组件持有子组件的 `Entity<T>`，可以直接操作子组件。

```rust
struct Parent {
    child: Entity<Child>,
}

struct Child {
    value: i32,
}

impl Parent {
    fn update_child(&mut self, cx: &mut Context<Self>) {
        // 直接更新子组件
        self.child.update(cx, |child, cx| {
            child.value = 100;
            cx.notify();
        });
    }
}

impl Render for Parent {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .child(self.child.clone())  // 渲染子组件
            .child(
                button("Update Child")
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.update_child(cx);
                    }))
            )
    }
}

// 创建组件树
fn create_tree(cx: &mut App) {
    let child = cx.new(|_| Child { value: 0 });
    let parent = cx.new(|_| Parent { child });
}
```

**适用场景**：
- ✅ 父组件完全控制子组件
- ✅ 单向数据流
- ❌ 子组件需要通知父组件

### 方式3: observe 观察变化

一个组件观察另一个组件的**所有变化**（只要调用了 `cx.notify()`）。

```rust
struct Counter {
    count: i32,
}

impl Counter {
    fn increment(&mut self, cx: &mut Context<Self>) {
        self.count += 1;
        cx.notify();  // ⭐ 触发所有 observer
    }
}

struct Observer {
    last_seen: i32,
}

impl Observer {
    fn new(counter: Entity<Counter>, cx: &mut Context<Self>) -> Self {
        // 观察 counter 的所有变化
        cx.observe(&counter, |this, counter_entity, cx| {
            // counter 调用 cx.notify() 时触发
            counter_entity.read(cx, |counter, _| {
                this.last_seen = counter.count;
                println!("Counter changed to: {}", counter.count);
            });
            cx.notify();  // 重新渲染自己
        }).detach();

        Self { last_seen: 0 }
    }
}
```

**适用场景**：
- ✅ 状态同步
- ✅ 被动响应变化
- ❌ 需要知道具体什么变了

### 方式4: EventEmitter + subscribe

发送**特定类型的事件**，比 `observe` 更精确。

#### 定义事件

```rust
// 1. 定义事件结构
struct CountChanged {
    old_value: i32,
    new_value: i32,
}

// 2. 声明组件可以发射这种事件
struct Counter {
    count: i32,
}

impl EventEmitter<CountChanged> for Counter {}

// 3. 发射事件
impl Counter {
    fn increment(&mut self, cx: &mut Context<Self>) {
        let old = self.count;
        self.count += 1;

        // 发射特定事件
        cx.emit(CountChanged {
            old_value: old,
            new_value: self.count,
        });
    }
}
```

#### 订阅事件

```rust
struct Subscriber {
    total_changes: i32,
    _subscription: Subscription,  // 必须持有，否则自动取消订阅
}

impl Subscriber {
    fn new(counter: Entity<Counter>, cx: &mut Context<Self>) -> Self {
        let subscription = cx.subscribe(&counter, |this, _counter, event, cx| {
            // 只在 CountChanged 事件时触发
            println!("Changed from {} to {}", event.old_value, event.new_value);
            this.total_changes += 1;
            cx.notify();
        });

        Self {
            total_changes: 0,
            _subscription: subscription,  // ⭐ 保持订阅活跃
        }
    }
}
```

**⚠️ 订阅生命周期管理**：

```rust
struct MyComponent {
    _subscriptions: Vec<Subscription>,  // 存储订阅
}

impl MyComponent {
    fn new(cx: &mut Context<Self>) -> Self {
        let mut subscriptions = Vec::new();

        subscriptions.push(cx.subscribe(&entity1, |...| { ... }));
        subscriptions.push(cx.subscribe(&entity2, |...| { ... }));

        Self { _subscriptions: subscriptions }
    }
}
// 当 MyComponent drop 时，subscriptions 自动清理
```

**适用场景**：
- ✅ 精确的事件通信
- ✅ 需要携带事件数据
- ✅ 类型安全的组件间通信

### 方式5: 父子双向通信

结合 Entity 引用和 EventEmitter，实现完整的父子通信。

```rust
// === 子组件：发射事件 ===
struct Button {
    label: String,
}

struct ButtonClicked {
    label: String,
}

impl EventEmitter<ButtonClicked> for Button {}

impl Button {
    fn handle_click(&mut self, cx: &mut Context<Self>) {
        cx.emit(ButtonClicked {
            label: self.label.clone(),
        });
    }
}

impl Render for Button {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .child(&self.label)
            .on_click(cx.listener(|this, _, _, cx| {
                this.handle_click(cx);
            }))
    }
}

// === 父组件：订阅事件 + 控制子组件 ===
struct Toolbar {
    buttons: Vec<Entity<Button>>,
    last_clicked: String,
    _subscriptions: Vec<Subscription>,
}

impl Toolbar {
    fn new(cx: &mut Context<Self>) -> Self {
        let mut buttons = Vec::new();
        let mut subscriptions = Vec::new();

        // 创建多个按钮
        for label in ["Save", "Load", "Exit"] {
            let button = cx.new(|_| Button {
                label: label.to_string(),
            });

            // 订阅每个按钮的事件（子 → 父）
            subscriptions.push(
                cx.subscribe(&button, |this, _button, event, cx| {
                    this.last_clicked = event.label.clone();
                    println!("Button '{}' clicked!", event.label);
                    cx.notify();
                })
            );

            buttons.push(button);
        }

        Self {
            buttons,
            last_clicked: String::new(),
            _subscriptions: subscriptions,
        }
    }

    // 父组件也可以更新子组件（父 → 子）
    fn disable_all_buttons(&mut self, cx: &mut Context<Self>) {
        for button in &self.buttons {
            button.update(cx, |btn, cx| {
                btn.label = format!("{} (disabled)", btn.label);
                cx.notify();
            });
        }
    }
}

impl Render for Toolbar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .child(format!("Last clicked: {}", self.last_clicked))
            .child(
                div()
                    .flex()
                    .gap_2()
                    .children(self.buttons.iter().cloned())
            )
            .child(
                button("Disable All")
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.disable_all_buttons(cx);
                    }))
            )
    }
}
```

### 方式对比表

| 方式 | 适用场景 | 优点 | 缺点 | 示例 |
|-----|---------|------|------|------|
| **Global** | 应用级配置 | 任何地方都能访问 | 难以追踪谁修改 | 主题、用户信息 |
| **Entity 引用** | 父→子单向控制 | 直接、简单 | 耦合度高 | 父组件完全控制子组件 |
| **observe** | 观察任何变化 | 自动响应 | 不知道具体什么变了 | 状态同步、镜像 |
| **subscribe** | 特定事件通信 | 类型安全、精确 | 需要定义事件 | 按钮点击、数据变化通知 |
| **父子双向** | 复杂交互 | 灵活、解耦 | 代码较多 | 工具栏和按钮 |

---

## 完整实例

### Todo 应用

一个完整的 Todo 应用，展示了 GPUI 的各种特性。

```rust
use gpui::*;

// ============ 全局状态 ============
struct AppSettings {
    show_completed: bool,
}

impl Global for AppSettings {}

// ============ 事件定义 ============
struct TodoAdded {
    text: String,
}

struct TodoRemoved {
    index: usize,
}

struct TodoToggled {
    index: usize,
}

// ============ Todo 模型 ============
struct TodoItem {
    text: String,
    completed: bool,
}

struct TodoList {
    items: Vec<TodoItem>,
}

impl EventEmitter<TodoAdded> for TodoList {}
impl EventEmitter<TodoRemoved> for TodoList {}
impl EventEmitter<TodoToggled> for TodoList {}

impl TodoList {
    fn new() -> Self {
        Self { items: Vec::new() }
    }

    fn add(&mut self, text: String, cx: &mut Context<Self>) {
        self.items.push(TodoItem {
            text: text.clone(),
            completed: false,
        });
        cx.emit(TodoAdded { text });
        cx.notify();
    }

    fn remove(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.items.len() {
            self.items.remove(index);
            cx.emit(TodoRemoved { index });
            cx.notify();
        }
    }

    fn toggle(&mut self, index: usize, cx: &mut Context<Self>) {
        if let Some(item) = self.items.get_mut(index) {
            item.completed = !item.completed;
            cx.emit(TodoToggled { index });
            cx.notify();
        }
    }
}

// ============ UI 组件 ============
struct TodoApp {
    todo_list: Entity<TodoList>,
    input_text: String,
    _subscription: Subscription,
}

impl TodoApp {
    fn new(cx: &mut Context<Self>) -> Self {
        let todo_list = cx.new(|_| TodoList::new());

        // 订阅 TodoList 的事件
        let subscription = cx.subscribe(&todo_list, |this, _list, event: &TodoAdded, cx| {
            println!("New todo added: {}", event.text);
            cx.notify();
        });

        Self {
            todo_list,
            input_text: String::new(),
            _subscription: subscription,
        }
    }

    fn add_todo(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        if !self.input_text.is_empty() {
            let text = self.input_text.clone();
            self.todo_list.update(cx, |list, cx| {
                list.add(text, cx);
            });
            self.input_text.clear();
            cx.notify();
        }
    }

    fn remove_todo(&mut self, index: usize, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.todo_list.update(cx, |list, cx| {
            list.remove(index, cx);
        });
    }

    fn toggle_todo(&mut self, index: usize, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.todo_list.update(cx, |list, cx| {
            list.toggle(index, cx);
        });
    }
}

impl Render for TodoApp {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let settings = cx.global::<AppSettings>();

        div()
            .flex()
            .flex_col()
            .gap_4()
            .p_4()
            // 标题
            .child(
                div()
                    .text_xl()
                    .font_bold()
                    .child("Todo List")
            )
            // 输入框
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(
                        // 注：这里简化了，实际需要使用 TextInput 组件
                        div()
                            .flex_1()
                            .border_1()
                            .p_2()
                            .child(&self.input_text)
                    )
                    .child(
                        button("Add")
                            .on_click(cx.listener(Self::add_todo))
                    )
            )
            // Todo 列表
            .child(
                self.todo_list.read(cx, |list, _| {
                    div()
                        .flex()
                        .flex_col()
                        .gap_2()
                        .children(
                            list.items.iter().enumerate()
                                .filter(|(_, item)| {
                                    settings.show_completed || !item.completed
                                })
                                .map(|(index, item)| {
                                    div()
                                        .flex()
                                        .justify_between()
                                        .items_center()
                                        .p_2()
                                        .border_1()
                                        .when(item.completed, |div| {
                                            div.bg(rgb(0xf0f0f0))
                                        })
                                        .child(
                                            div()
                                                .flex()
                                                .gap_2()
                                                .child(
                                                    button(if item.completed { "✓" } else { "○" })
                                                        .on_click(cx.listener(move |this, event, window, cx| {
                                                            this.toggle_todo(index, event, window, cx);
                                                        }))
                                                )
                                                .child(
                                                    div()
                                                        .when(item.completed, |div| {
                                                            div.text_color(rgb(0x888888))
                                                        })
                                                        .child(&item.text)
                                                )
                                        )
                                        .child(
                                            button("Delete")
                                                .on_click(cx.listener(move |this, event, window, cx| {
                                                    this.remove_todo(index, event, window, cx);
                                                }))
                                        )
                                })
                        )
                })
            )
    }
}

// ============ 主函数 ============
fn main() {
    Application::new().run(|cx: &mut App| {
        // 初始化全局状态
        cx.set_global(AppSettings {
            show_completed: true,
        });

        // 打开窗口
        cx.open_window(
            WindowOptions::default(),
            |window, cx| {
                cx.new(|cx| TodoApp::new(cx))
            }
        ).unwrap();
    });
}
```

---

## 最佳实践

### 1. 状态管理

✅ **DO**:
- 状态修改后立即调用 `cx.notify()`
- 使用 `Entity<T>` 持有其他组件的引用
- 将订阅存储在 `_subscription` 字段中（前缀 `_` 表示不会被直接使用）

❌ **DON'T**:
- 不要忘记调用 `cx.notify()`
- 不要在 `render()` 中修改状态
- 不要忘记持有 `Subscription`（否则立即取消订阅）

```rust
// ❌ 错误
struct MyComponent {
    other: Entity<Other>,
}

impl MyComponent {
    fn new(other: Entity<Other>, cx: &mut Context<Self>) -> Self {
        cx.subscribe(&other, |this, _, event, cx| {
            // ...
        });  // ⚠️ 订阅立即被 drop，不会触发！

        Self { other }
    }
}

// ✅ 正确
struct MyComponent {
    other: Entity<Other>,
    _subscription: Subscription,  // 持有订阅
}

impl MyComponent {
    fn new(other: Entity<Other>, cx: &mut Context<Self>) -> Self {
        let subscription = cx.subscribe(&other, |this, _, event, cx| {
            // ...
        });

        Self {
            other,
            _subscription: subscription,
        }
    }
}
```

### 2. 组件通信

**选择合适的通信方式**：

```rust
// 应用主题 → Global
cx.set_global(Theme { dark: true });

// 父控制子 → Entity 引用
parent.child.update(cx, |child, cx| {
    child.value = 100;
    cx.notify();
});

// 状态同步 → observe
cx.observe(&model, |this, model, cx| {
    this.sync_from_model(model.read(cx));
    cx.notify();
});

// 事件通信 → EventEmitter + subscribe
cx.subscribe(&button, |this, btn, event: &Clicked, cx| {
    this.handle_button_click(event);
    cx.notify();
});
```

### 3. 性能优化

```rust
// 1. 避免不必要的 notify
impl Counter {
    fn set_value(&mut self, new_value: i32, cx: &mut Context<Self>) {
        if self.count != new_value {  // ✅ 检查是否真的变化
            self.count = new_value;
            cx.notify();
        }
    }
}

// 2. 使用 .cached() 缓存视图
impl Render for ExpensiveView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .child(
                self.expensive_child.clone().into_any_view().cached(Style::default())
                // 如果没有调用 cx.notify()，会重用上一帧的渲染结果
            )
    }
}

// 3. 批量更新
impl TodoList {
    fn add_multiple(&mut self, items: Vec<String>, cx: &mut Context<Self>) {
        for item in items {
            self.items.push(TodoItem { text: item, completed: false });
        }
        cx.notify();  // ✅ 只 notify 一次
    }
}
```

### 4. 错误处理

```rust
// Entity 可能已经被释放
impl MyComponent {
    fn try_update_other(&mut self, cx: &mut Context<Self>) {
        // 使用 WeakEntity 避免循环引用
        if let Some(other) = self.weak_other.upgrade() {
            other.update(cx, |other, cx| {
                other.value = 100;
                cx.notify();
            });
        } else {
            println!("Other entity was released");
        }
    }
}
```

### 5. 类型安全

```rust
// 使用类型系统避免错误
struct UserId(u64);
struct PostId(u64);

struct User {
    id: UserId,  // ✅ 不会混淆
}

struct Post {
    id: PostId,
    author: UserId,  // ✅ 类型安全
}
```

### 6. 项目结构

```
src/
├── main.rs              # 应用入口
├── app.rs              # App 全局状态
├── components/         # UI 组件
│   ├── mod.rs
│   ├── button.rs
│   └── input.rs
├── models/             # 数据模型
│   ├── mod.rs
│   ├── todo.rs
│   └── user.rs
└── events.rs           # 事件定义
```

---

## 总结

### 核心概念回顾

| 概念 | 作用 | 关键方法 |
|-----|------|---------|
| **AppContext** | 统一接口 | `new()`, `update_entity()`, `read_global()` |
| **App** | 全局管理器 | 存储所有实体、窗口、全局状态 |
| **Context<T>** | 实体上下文 | `notify()`, `emit()`, `observe()`, `subscribe()` |
| **Entity<T>** | 智能指针 | `read()`, `update()`, `downgrade()` |

### 状态管理公式

```
状态定义(struct) → 实现Render → cx.listener()绑定事件 →
修改状态 → cx.notify() → 重新render() → UI更新
```

### 跨组件通信决策树

```
需要跨组件通信？
│
├─ 应用级配置？ → Global
│
├─ 父组件完全控制子组件？ → Entity 引用
│
├─ 需要精确的事件类型？ → EventEmitter + subscribe
│
├─ 只需要知道变化了？ → observe
│
└─ 复杂的双向通信？ → 组合使用
```

### 常见模式

1. **单例模式** - 使用 `Global`
2. **观察者模式** - 使用 `observe()` / `subscribe()`
3. **发布订阅模式** - 使用 `EventEmitter` + `subscribe()`
4. **组合模式** - 使用 `Entity<T>` 嵌套
5. **命令模式** - 使用 `cx.listener()`

---

## 参考资源

- [GPUI 官方文档](https://www.gpui.rs/)
- [Zed 编辑器源码](https://github.com/zed-industries/zed) - GPUI 的实际应用
- [GPUI Examples](https://github.com/zed-industries/zed/tree/main/crates/gpui/examples)

---

**文档版本**: v1.0
**最后更新**: 2025-12-29
**适用 GPUI 版本**: 0.2.x
