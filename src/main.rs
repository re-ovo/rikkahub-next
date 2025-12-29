use gpui::*;
use gpui::prelude::FluentBuilder;
use gpui_component::{button::*, *};

// Todo数据项
#[derive(Clone)]
struct TodoItem {
    text: String,
    completed: bool,
}

// Todo应用主组件
pub struct TodoApp {
    items: Vec<TodoItem>,
}

impl TodoApp {
    fn new() -> Self {
        // 添加一些示例待办事项
        Self {
            items: vec![
                TodoItem {
                    text: "学习 GPUI 框架".to_string(),
                    completed: false,
                },
                TodoItem {
                    text: "编写 Todo 应用".to_string(),
                    completed: true,
                },
                TodoItem {
                    text: "添加更多功能".to_string(),
                    completed: false,
                },
            ],
        }
    }

    // 添加新的示例Todo
    fn add_sample_todo(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.items.push(TodoItem {
            text: format!("新任务 #{}", self.items.len() + 1),
            completed: false,
        });
        cx.notify();
    }

    // 删除Todo
    fn remove_todo(&mut self, index: usize, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        if index < self.items.len() {
            self.items.remove(index);
            cx.notify();
        }
    }

    // 切换完成状态
    fn toggle_todo(&mut self, index: usize, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(item) = self.items.get_mut(index) {
            item.completed = !item.completed;
            cx.notify();
        }
    }
}

impl Render for TodoApp {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let total = self.items.len();
        let completed = self.items.iter().filter(|item| item.completed).count();
        let pending = total - completed;

        div()
            .v_flex()
            .gap_3()
            .size_full()
            .p_4()
            .bg(rgb(0xf5f5f5))
            // 标题
            .child(
                div()
                    .text_2xl()
                    .font_bold()
                    .text_color(rgb(0x333333))
                    .child("📝 Todo 应用")
            )
            // 添加按钮区域
            .child(
                div()
                    .h_flex()
                    .gap_2()
                    .child(
                        Button::new("add-btn")
                            .primary()
                            .label("添加示例任务")
                            .on_click(cx.listener(Self::add_sample_todo))
                    )
            )
            // 统计信息
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0x666666))
                    .child(format!(
                        "总计: {} | 已完成: {} | 未完成: {}",
                        total, completed, pending
                    ))
            )
            // Todo列表
            .child(
                div()
                    .v_flex()
                    .gap_2()
                    .flex_1()
                    .children(
                        self.items.iter().enumerate().map(|(index, item)| {
                            div()
                                .h_flex()
                                .gap_3()
                                .items_center()
                                .p_3()
                                .border_1()
                                .border_color(rgb(0xdddddd))
                                .rounded_md()
                                .bg(rgb(0xffffff))
                                .when(item.completed, |el| {
                                    el.bg(rgb(0xe8f5e9))
                                })
                                // 完成状态按钮
                                .child(
                                    Button::new(("toggle", index))
                                        .label(if item.completed { "✓" } else { "○" })
                                        .on_click(cx.listener(move |this, event, window, cx| {
                                            this.toggle_todo(index, event, window, cx);
                                        }))
                                )
                                // Todo文本
                                .child(
                                    div()
                                        .flex_1()
                                        .text_color(if item.completed {
                                            rgb(0x888888)
                                        } else {
                                            rgb(0x333333)
                                        })
                                        .when(item.completed, |el| {
                                            el.font_weight(FontWeight::LIGHT)
                                        })
                                        .child(item.text.clone())
                                )
                                // 删除按钮
                                .child(
                                    Button::new(("delete", index))
                                        .ghost()
                                        .label("删除")
                                        .on_click(cx.listener(move |this, event, window, cx| {
                                            this.remove_todo(index, event, window, cx);
                                        }))
                                )
                        })
                    )
            )
    }
}

fn main() {
    let app = Application::new();

    app.run(move |cx| {
        // This must be called before using any GPUI Component features.
        gpui_component::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|_| TodoApp::new());
                // This first level on the window, should be a Root.
                cx.new(|cx| Root::new(view, window, cx))
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}