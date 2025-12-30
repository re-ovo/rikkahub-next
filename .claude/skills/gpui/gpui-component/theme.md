# 主题系统

## 概述

gpui-component 提供了一套完整的主题系统，支持明暗主题切换、自定义主题配置、主题热更新、颜色管理和系统外观同步。主题系统包括：

- **全局主题管理**：使用 GPUI Global 机制管理全局主题状态
- **主题模式支持**：Light（浅色）和 Dark（深色）两种主题模式
- **丰富的颜色系统**：包含 80+ 个语义化颜色，支持主题切换时自动应用
- **灵活的配置**：支持通过 JSON 文件定义自定义主题
- **颜色操作工具**：提供颜色解析、混合、亮度调整等便捷方法
- **自动同步**：自动同步系统外观，监听系统深色模式变化
- **热更新机制**：支持监听主题文件夹，实时加载新主题

## API 定义

### 主题结构

```rust
/// 全局主题配置
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Theme {
    // 主题颜色
    pub colors: ThemeColor,
    // 代码高亮主题
    pub highlight_theme: Arc<HighlightTheme>,
    // 浅色主题配置
    pub light_theme: Rc<ThemeConfig>,
    // 深色主题配置
    pub dark_theme: Rc<ThemeConfig>,
    // 当前主题模式
    pub mode: ThemeMode,
    // 应用字体族（默认：.SystemUIFont）
    pub font_family: SharedString,
    // 基础字体大小（默认：16px）
    pub font_size: Pixels,
    // 等宽字体族（macOS: Menlo, Windows: Consolas, Linux: DejaVu Sans Mono）
    pub mono_font_family: SharedString,
    // 等宽字体大小（默认：13px）
    pub mono_font_size: Pixels,
    // 通用元素圆角半径（默认：6px）
    pub radius: Pixels,
    // 大型元素圆角半径（默认：8px），用于 Dialog、Notification 等
    pub radius_lg: Pixels,
    // 是否启用阴影效果（默认：true）
    pub shadow: bool,
    // 透明色
    pub transparent: Hsla,
    // 滚动条显示模式
    pub scrollbar_show: ScrollbarShow,
    // 通知设置
    pub notification: NotificationSettings,
    // Tile 网格大小（默认：4px）
    pub tile_grid_size: Pixels,
    // Tile 面板阴影（默认：true）
    pub tile_shadow: bool,
    // Tile 面板圆角（默认：0px）
    pub tile_radius: Pixels,
    // List 相关设置
    pub list: ListSettings,
}
```

### 主题模式枚举

```rust
/// 主题模式：浅色或深色
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ThemeMode {
    #[default]
    Light,  // 浅色主题
    Dark,   // 深色主题
}

impl ThemeMode {
    // 返回 true 如果当前是深色主题
    pub fn is_dark(&self) -> bool;
    // 返回主题名称的小写字符串："light" 或 "dark"
    pub fn name(&self) -> &'static str;
}
```

### 主题颜色结构

```rust
/// 主题颜色定义，包含所有UI组件所用的语义化颜色
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, JsonSchema)]
pub struct ThemeColor {
    // 主要颜色及变体
    pub primary: Hsla,
    pub primary_hover: Hsla,
    pub primary_active: Hsla,
    pub primary_foreground: Hsla,

    // 次要颜色及变体
    pub secondary: Hsla,
    pub secondary_hover: Hsla,
    pub secondary_active: Hsla,
    pub secondary_foreground: Hsla,

    // 基础颜色
    pub background: Hsla,
    pub foreground: Hsla,
    pub border: Hsla,

    // 状态颜色
    pub success: Hsla,
    pub success_hover: Hsla,
    pub success_active: Hsla,
    pub success_foreground: Hsla,

    pub danger: Hsla,
    pub danger_hover: Hsla,
    pub danger_active: Hsla,
    pub danger_foreground: Hsla,

    pub warning: Hsla,
    pub warning_hover: Hsla,
    pub warning_active: Hsla,
    pub warning_foreground: Hsla,

    pub info: Hsla,
    pub info_hover: Hsla,
    pub info_active: Hsla,
    pub info_foreground: Hsla,

    // 中立颜色
    pub accent: Hsla,
    pub accent_foreground: Hsla,
    pub muted: Hsla,
    pub muted_foreground: Hsla,

    // UI 组件特定颜色
    pub input: Hsla,
    pub link: Hsla,
    pub link_active: Hsla,
    pub link_hover: Hsla,
    pub selection: Hsla,
    pub ring: Hsla,

    // List 和 Table 颜色
    pub list: Hsla,
    pub list_active: Hsla,
    pub list_hover: Hsla,
    pub list_even: Hsla,
    pub list_head: Hsla,

    pub table: Hsla,
    pub table_active: Hsla,
    pub table_hover: Hsla,
    pub table_even: Hsla,
    pub table_head: Hsla,
    pub table_head_foreground: Hsla,

    // Sidebar 颜色
    pub sidebar: Hsla,
    pub sidebar_accent: Hsla,
    pub sidebar_accent_foreground: Hsla,
    pub sidebar_border: Hsla,
    pub sidebar_foreground: Hsla,
    pub sidebar_primary: Hsla,
    pub sidebar_primary_foreground: Hsla,

    // 其他组件颜色
    pub accordion: Hsla,
    pub accordion_hover: Hsla,
    pub group_box: Hsla,
    pub group_box_foreground: Hsla,
    pub caret: Hsla,
    pub popover: Hsla,
    pub popover_foreground: Hsla,
    pub scrollbar: Hsla,
    pub scrollbar_thumb: Hsla,
    pub scrollbar_thumb_hover: Hsla,
    pub slider_bar: Hsla,
    pub slider_thumb: Hsla,
    pub switch: Hsla,
    pub switch_thumb: Hsla,
    pub skeleton: Hsla,
    pub tab: Hsla,
    pub tab_active: Hsla,
    pub tab_foreground: Hsla,
    pub progress_bar: Hsla,
    pub overlay: Hsla,
    pub title_bar: Hsla,
    pub title_bar_border: Hsla,

    // 图表颜色
    pub chart_1: Hsla,
    pub chart_2: Hsla,
    pub chart_3: Hsla,
    pub chart_4: Hsla,
    pub chart_5: Hsla,
    pub bullish: Hsla,   // K线图上升颜色
    pub bearish: Hsla,   // K线图下降颜色

    // 基础颜色（支持 light 变体）
    pub red: Hsla,
    pub red_light: Hsla,
    pub green: Hsla,
    pub green_light: Hsla,
    pub blue: Hsla,
    pub blue_light: Hsla,
    pub yellow: Hsla,
    pub yellow_light: Hsla,
    pub magenta: Hsla,
    pub magenta_light: Hsla,
    pub cyan: Hsla,
    pub cyan_light: Hsla,
}
```

### 主题配置结构

```rust
/// 主题配置文件的定义
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
#[serde(default)]
pub struct ThemeConfig {
    // 该主题是否为默认主题
    pub is_default: bool,
    // 主题名称
    pub name: SharedString,
    // 主题模式（Light 或 Dark）
    pub mode: ThemeMode,

    // 字体配置
    #[serde(rename = "font.size")]
    pub font_size: Option<f32>,
    #[serde(rename = "font.family")]
    pub font_family: Option<SharedString>,
    #[serde(rename = "mono_font.family")]
    pub mono_font_family: Option<SharedString>,
    #[serde(rename = "mono_font.size")]
    pub mono_font_size: Option<f32>,

    // 样式配置
    #[serde(rename = "radius")]
    pub radius: Option<usize>,
    #[serde(rename = "radius.lg")]
    pub radius_lg: Option<usize>,
    #[serde(rename = "shadow")]
    pub shadow: Option<bool>,

    // 颜色和高亮配置
    pub colors: ThemeConfigColors,
    pub highlight: Option<HighlightThemeStyle>,
}

/// 主题配置中的颜色定义
#[derive(Debug, Default, Clone, JsonSchema, Serialize, Deserialize)]
pub struct ThemeConfigColors {
    // 支持所有 ThemeColor 中的字段，使用特定的 serde rename
    // 例如：
    #[serde(rename = "primary.background")]
    pub primary: Option<SharedString>,
    #[serde(rename = "primary.hover.background")]
    pub primary_hover: Option<SharedString>,
    // ... 更多字段
}
```

### 主题注册表

```rust
/// 主题注册表，管理所有可用的主题
#[derive(Default, Debug)]
pub struct ThemeRegistry {
    themes_dir: PathBuf,
    default_themes: HashMap<ThemeMode, Rc<ThemeConfig>>,
    themes: HashMap<SharedString, Rc<ThemeConfig>>,
    has_custom_themes: bool,
}
```

### ActiveTheme 特性

```rust
/// 用于获取当前活跃主题的特性
pub trait ActiveTheme {
    fn theme(&self) -> &Theme;
}

// 为 App 实现了 ActiveTheme
impl ActiveTheme for App {
    fn theme(&self) -> &Theme {
        Theme::global(self)
    }
}
```

### 颜色操作特性

```rust
/// 提供颜色操作的特性
pub trait Colorize: Sized {
    // 设置颜色透明度（0.0 到 1.0）
    fn opacity(&self, opacity: f32) -> Self;
    // 将每个颜色通道除以除数
    fn divide(&self, divisor: f32) -> Self;
    // 反转颜色
    fn invert(&self) -> Self;
    // 反转亮度值
    fn invert_l(&self) -> Self;
    // 增加亮度（factor: 0.0 到 1.0）
    fn lighten(&self, amount: f32) -> Self;
    // 降低亮度（factor: 0.0 到 1.0）
    fn darken(&self, amount: f32) -> Self;
    // 应用另一个颜色的色调和饱和度，保持自身亮度
    fn apply(&self, base_color: Self) -> Self;
    // 混合两个颜色（factor: 0.0 到 1.0，表示第一个颜色的权重）
    fn mix(&self, other: Self, factor: f32) -> Self;
    // 改变色调（范围：0.0 到 1.0）
    fn hue(&self, hue: f32) -> Self;
    // 改变饱和度（范围：0.0 到 1.0）
    fn saturation(&self, saturation: f32) -> Self;
    // 改变亮度（范围：0.0 到 1.0）
    fn lightness(&self, lightness: f32) -> Self;
    // 转换为十六进制字符串
    fn to_hex(&self) -> String;
    // 从十六进制字符串解析颜色
    fn parse_hex(hex: &str) -> Result<Self>;
}
```

## 主要方法/属性

### Theme 全局方法

| 方法 | 参数类型 | 说明 |
|------|---------|------|
| `global(cx)` | `&App` | 获取全局主题的只读引用 |
| `global_mut(cx)` | `&mut App` | 获取全局主题的可变引用 |
| `is_dark()` | - | 返回当前主题是否为深色主题 |
| `theme_name()` | - | 获取当前活跃主题的名称 |
| `change(mode, window, cx)` | `ThemeMode, Option<&mut Window>, &mut App` | 切换主题模式 |
| `sync_system_appearance(window, cx)` | `Option<&mut Window>, &mut App` | 同步系统外观设置 |
| `sync_scrollbar_appearance(cx)` | `&mut App` | 同步滚动条显示行为 |
| `apply_config(config)` | `&Rc<ThemeConfig>` | 应用主题配置 |
| `init(cx)` | `&mut App` | 初始化主题系统 |

### ThemeRegistry 方法

| 方法 | 参数类型 | 说明 |
|------|---------|------|
| `global(cx)` | `&App` | 获取主题注册表 |
| `global_mut(cx)` | `&mut App` | 获取主题注册表的可变引用 |
| `watch_dir(themes_dir, cx, on_load)` | `PathBuf, &mut App, F` | 监听主题文件夹，自动加载新主题 |
| `themes()` | - | 获取所有主题的映射 |
| `sorted_themes()` | - | 获取排序后的主题列表（默认优先，浅色优先，名称排序） |
| `default_themes()` | - | 获取默认主题映射 |
| `default_light_theme()` | - | 获取默认浅色主题 |
| `default_dark_theme()` | - | 获取默认深色主题 |

### ThemeColor 方法

| 方法 | 参数类型 | 说明 |
|------|---------|------|
| `light()` | - | 获取默认浅色主题颜色（Arc） |
| `dark()` | - | 获取默认深色主题颜色（Arc） |

### 颜色解析函数

```rust
// 创建 HSL 颜色（值范围：h: 0-360, s: 0-100, l: 0-100）
pub fn hsl(h: f32, s: f32, l: f32) -> Hsla;

// 尝试解析颜色（支持 HEX、Tailwind 颜色格式）
pub fn try_parse_color(color: &str) -> Result<Hsla>;

// 预定义的颜色函数
pub fn black() -> Hsla;
pub fn white() -> Hsla;
pub fn red_500() -> Hsla;
pub fn blue_600() -> Hsla;
// ... 以及其他 ColorName 对应的颜色（支持 scale 50-950）
```

### ColorName 枚举和方法

```rust
/// 预定义的颜色名称
pub enum ColorName {
    Gray, Red, Orange, Amber, Yellow, Lime, Green, Emerald,
    Teal, Cyan, Sky, Blue, Indigo, Violet, Purple, Fuchsia,
    Pink, Rose,
}

impl ColorName {
    // 获取所有可用颜色名称
    pub fn all() -> [Self; 18];
    // 根据 scale 获取颜色（scale: 50, 100, 200, ..., 900, 950）
    pub fn scale(&self, scale: usize) -> Hsla;
}
```

## 使用示例

### 基础用法：获取和使用全局主题

```rust
use gpui_component::{Theme, ActiveTheme};

fn render_with_theme(cx: &App) {
    let theme = Theme::global(cx);

    // 检查当前主题模式
    if theme.is_dark() {
        println!("当前使用深色主题");
    } else {
        println!("当前使用浅色主题");
    }

    // 访问主题颜色
    let primary_color = theme.primary;
    let background_color = theme.background;
    let foreground_color = theme.foreground;

    // 使用 ActiveTheme trait
    let theme = cx.theme();  // 等同于 Theme::global(cx)
}
```

### 主题切换示例

```rust
use gpui::{App, Window};
use gpui_component::{Theme, ThemeMode};

fn toggle_theme(window: &mut Window, cx: &mut App) {
    let current_mode = Theme::global(cx).mode;
    let new_mode = if current_mode.is_dark() {
        ThemeMode::Light
    } else {
        ThemeMode::Dark
    };

    // 切换主题并刷新窗口
    Theme::change(new_mode, Some(window), cx);
}

fn switch_to_dark(window: &mut Window, cx: &mut App) {
    Theme::change(ThemeMode::Dark, Some(window), cx);
}
```

### 同步系统外观

```rust
use gpui_component::Theme;

fn sync_system_appearance(window: Option<&mut Window>, cx: &mut App) {
    // 根据系统外观自动切换主题
    Theme::sync_system_appearance(window, cx);
}

fn on_system_appearance_changed(window: &mut Window, cx: &mut App) {
    // 当系统外观改变时调用
    Theme::sync_system_appearance(Some(window), cx);
}
```

### 颜色操作示例

```rust
use gpui_component::{Colorize, hsl};

fn color_operations() {
    // 创建颜色
    let primary = hsl(220.0, 90.0, 50.0);  // 深蓝色

    // 调整透明度
    let transparent_primary = primary.opacity(0.5);

    // 亮度调整
    let lighter_primary = primary.lighten(0.2);
    let darker_primary = primary.darken(0.2);

    // 混合颜色
    let secondary = hsl(40.0, 100.0, 50.0);  // 黄色
    let mixed = primary.mix(secondary, 0.5);  // 50% 混合

    // 颜色反转
    let inverted = primary.invert();

    // 十六进制转换
    let hex_str = primary.to_hex();  // "#0066FF"

    // 从十六进制解析
    let parsed = Hsla::parse_hex("#0066FF").unwrap();
}
```

### 使用 Tailwind 颜色

```rust
use gpui_component::{try_parse_color, ColorName};

fn use_tailwind_colors() -> anyhow::Result<()> {
    // 直接使用 ColorName
    let red = ColorName::Red.scale(500);
    let blue_600 = ColorName::Blue.scale(600);

    // 或使用字符串解析
    let red = try_parse_color("red")?;  // 默认使用 500 scale
    let blue_600 = try_parse_color("blue-600")?;
    let pink_with_opacity = try_parse_color("pink/50")?;  // 50% 透明度
    let orange_300_50_opacity = try_parse_color("orange-300/50")?;

    // 或使用十六进制
    let custom = try_parse_color("#FF0000")?;
    let custom_with_alpha = try_parse_color("#FF0000AA")?;

    Ok(())
}
```

### 自定义主题配置（JSON）

```json
{
  "themes": [
    {
      "is_default": true,
      "name": "My Custom Light Theme",
      "mode": "light",
      "font.size": 14,
      "font.family": "Segoe UI",
      "radius": 8,
      "shadow": true,
      "colors": {
        "background": "#FFFFFF",
        "foreground": "#000000",
        "primary.background": "#0066FF",
        "primary.foreground": "#FFFFFF",
        "primary.hover.background": "#0052CC",
        "primary.active.background": "#003D99",
        "secondary.background": "#E0E0E0",
        "success.background": "#10B981",
        "danger.background": "#EF4444",
        "warning.background": "#F59E0B",
        "info.background": "#3B82F6"
      },
      "highlight": {
        "style": {
          "editor_background": "#FFFFFF"
        }
      }
    }
  ]
}
```

### 加载自定义主题文件夹

```rust
use gpui::{App};
use gpui_component::ThemeRegistry;
use std::path::PathBuf;

fn setup_custom_themes(cx: &mut App) -> anyhow::Result<()> {
    let themes_dir = PathBuf::from("./themes");

    // 监听主题文件夹，自动加载新主题
    ThemeRegistry::watch_dir(themes_dir, cx, |cx| {
        println!("主题已加载");
    })?;

    Ok(())
}
```

### 获取当前主题信息

```rust
use gpui_component::{Theme, ThemeRegistry};

fn print_theme_info(cx: &App) {
    let theme = Theme::global(cx);
    let registry = ThemeRegistry::global(cx);

    // 获取当前主题名称
    let current_name = theme.theme_name();
    println!("当前主题: {}", current_name);

    // 获取所有可用主题
    let all_themes = registry.sorted_themes();
    for theme_config in all_themes {
        println!("  - {} ({:?})", theme_config.name, theme_config.mode);
    }

    // 获取默认主题
    let light_theme = registry.default_light_theme();
    let dark_theme = registry.default_dark_theme();

    println!("默认浅色主题: {}", light_theme.name);
    println!("默认深色主题: {}", dark_theme.name);
}
```

### 应用主题配置

```rust
use gpui_component::{Theme, ThemeRegistry};

fn apply_custom_theme(theme_name: &str, cx: &mut App) {
    let registry = ThemeRegistry::global(cx);

    if let Some(theme_config) = registry.themes().get(theme_name) {
        // 应用主题配置
        Theme::global_mut(cx).apply_config(&theme_config.clone());
        println!("已应用主题: {}", theme_name);
    } else {
        println!("未找到主题: {}", theme_name);
    }
}
```

### 主题初始化

```rust
use gpui::{App};
use gpui_component::Theme;

fn initialize_app() {
    let mut app = App::new();

    // 初始化主题系统
    // 这会设置默认主题并根据系统外观选择浅色或深色主题
    Theme::init(&mut app);

    // 现在主题已就绪，可以访问
    let theme = Theme::global(&app);
    println!("初始化完成，当前主题: {}", theme.theme_name());
}
```

## 进阶用法

### 响应主题变化

```rust
use gpui::{App};
use gpui_component::Theme;

fn setup_theme_observer(cx: &mut App) {
    cx.observe_global::<Theme>(|cx| {
        let theme = Theme::global(cx);
        println!("主题已改变!");
        println!("当前模式: {:?}", theme.mode);
        println!("当前主题: {}", theme.theme_name());

        // 刷新 UI
        cx.refresh_windows();
    })
    .detach();
}
```

### 自定义主题应用逻辑

```rust
use gpui_component::{Theme, ThemeConfig, Rc};
use gpui::{App, Window};

fn create_and_apply_theme(config: ThemeConfig, window: &mut Window, cx: &mut App) {
    let theme_rc = Rc::new(config);

    // 应用配置
    Theme::global_mut(cx).apply_config(&theme_rc);

    // 刷新窗口显示
    window.refresh();
}
```

### 动态颜色计算

```rust
use gpui_component::{Colorize, Theme};

fn calculate_component_color(cx: &App) -> gpui::Hsla {
    let theme = Theme::global(cx);

    // 根据主题模式动态调整颜色
    if theme.is_dark() {
        // 深色主题中使用较浅的颜色
        theme.primary.lighten(0.2)
    } else {
        // 浅色主题中使用较深的颜色
        theme.primary.darken(0.1)
    }
}
```

## 最佳实践

### 1. 使用语义化颜色而非硬编码颜色

```rust
// ✓ 好的做法：使用主题颜色
let text_color = theme.foreground;
let error_color = theme.danger;

// ✗ 避免：硬编码颜色值
let text_color = hsla(0.0, 0.0, 0.0, 1.0);  // 黑色
```

### 2. 主题初始化应在应用启动时进行

```rust
// 在 main 或应用初始化中调用
Theme::init(&mut app);
```

### 3. 使用 ActiveTheme trait 简化代码

```rust
// ✓ 好的做法
fn render(cx: &App) {
    let theme = cx.theme();
}

// ✗ 避免
fn render(cx: &App) {
    let theme = Theme::global(cx);
}
```

### 4. 主题文件夹监听应在应用启动时设置

```rust
// 在应用初始化中
ThemeRegistry::watch_dir(themes_dir, cx, |_cx| {
    // 回调在主题加载完成时执行
})?;
```

### 5. 颜色操作链式调用

```rust
let adjusted_color = theme.primary
    .opacity(0.8)
    .lighten(0.1)
    .mix(theme.secondary, 0.2);
```

### 6. 为自定义组件定义颜色映射

```rust
fn get_button_color(theme: &Theme, status: ButtonStatus) -> Hsla {
    match status {
        ButtonStatus::Success => theme.success,
        ButtonStatus::Warning => theme.warning,
        ButtonStatus::Error => theme.danger,
        ButtonStatus::Default => theme.primary,
    }
}
```

## 注意事项

### 1. 颜色值范围

- Hue (h): 0.0 - 1.0 或 0 - 360（取决于使用的函数）
- Saturation (s): 0.0 - 1.0 或 0 - 100
- Lightness (l): 0.0 - 1.0 或 0 - 100
- Alpha (a): 0.0 - 1.0 (0 完全透明，1 完全不透明)

### 2. 主题切换时的刷新

```rust
// 切换主题后一定要刷新窗口，否则 UI 不会更新
Theme::change(ThemeMode::Dark, Some(window), cx);
window.refresh();  // 或在 Theme::change 中自动调用
```

### 3. 主题配置文件格式

- 必须是有效的 JSON
- `colors` 对象中的键必须使用指定的 serde rename（例如 `primary.background` 而非 `primary`）
- 颜色值可以是 HEX 格式（#RRGGBB 或 #RRGGBBAA）或 Tailwind 格式（red、blue-600 等）

### 4. 系统外观同步

- Linux 平台上 `window.appearance()` 可能返回错误，建议使用 `cx.window_appearance()` 备选
- 监听系统深色模式变化需要在事件处理中调用 `Theme::sync_system_appearance()`

### 5. 主题注册表的线程安全

- ThemeRegistry 是全局的，在多线程环境中访问时需要确保线程安全
- 大多数操作通过 `cx.update()` 回调保证在主线程执行

### 6. 默认主题的重要性

- 始终定义默认浅色和深色主题，作为颜色值的备选方案
- 自定义主题如果省略某些颜色定义，会自动使用默认主题的对应颜色

### 7. 高亮主题

- 如果主题配置中定义了 `highlight` 部分，它将覆盖代码编辑器的高亮样式
- 支持与 Zed 编辑器兼容的 `HighlightThemeStyle` 定义

### 8. 性能考虑

- 频繁访问 `Theme::global()` 时可以缓存引用以减少全局查询开销
- 大型应用可以在初始化时预计算常用颜色值

## 常见颜色格式

### HEX 格式

```rust
// 6位十六进制（RGB）
try_parse_color("#FF0000")?;   // 红色

// 8位十六进制（RGBA）
try_parse_color("#FF0000AA")?; // 红色 + 67% 透明度
```

### Tailwind 格式

```rust
// 基础色
try_parse_color("red")?;       // red-500

// 指定 scale
try_parse_color("blue-600")?;

// 带透明度
try_parse_color("green/50")?;  // 50% 透明度
try_parse_color("yellow-300/75")?;
```

### HSL 函数

```rust
// hsl(hue, saturation, lightness)
// hue: 0-360, saturation: 0-100, lightness: 0-100
hsl(220.0, 90.0, 50.0);
```

## 相关链接

- [ColorName 和颜色方案](../../../gpui-component/crates/ui/src/theme/color.rs)
- [默认主题定义](../../../gpui-component/crates/ui/src/theme/default-theme.json)
- [Tailwind 颜色参考](https://tailwindcss.com/docs/colors)
