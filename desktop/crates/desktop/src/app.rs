//! 主应用组件

use gpui::*;

pub struct RikkaApp {
    // TODO: 添加状态
}

impl RikkaApp {
    pub fn new() -> Self {
        Self {}
    }
}

impl Render for RikkaApp {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x1e1e2e))
            .text_color(rgb(0xcdd6f4))
            .child(
                // 标题栏
                div()
                    .h(px(48.0))
                    .px_4()
                    .flex()
                    .items_center()
                    .border_b_1()
                    .border_color(rgb(0x313244))
                    .child("RikkaHub"),
            )
            .child(
                // 主内容区
                div()
                    .flex_1()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child("欢迎使用 RikkaHub"),
            )
    }
}
