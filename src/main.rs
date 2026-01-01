use std::path::PathBuf;

use gpui::*;
use gpui::prelude::FluentBuilder;
use gpui_component::{button::*, scroll::ScrollableElement, *};

// 会话数据
#[derive(Clone)]
struct Conversation {
    name: String,
    avatar: String,
    last_message: String,
    time: String,
}

// 消息数据
#[derive(Clone)]
struct Message {
    content: String,
    is_self: bool,
    time: String,
}

// 聊天应用主组件
pub struct ChatApp {
    conversations: Vec<Conversation>,
    selected_conversation: Option<usize>,
    messages: Vec<Message>,
}

impl ChatApp {
    fn new() -> Self {
        Self {
            conversations: vec![
                Conversation {
                    name: "Alice".to_string(),
                    avatar: "👩".to_string(),
                    last_message: "晚上一起吃饭吗？".to_string(),
                    time: "18:30".to_string(),
                },
                Conversation {
                    name: "Bob".to_string(),
                    avatar: "👨".to_string(),
                    last_message: "好的，没问题".to_string(),
                    time: "17:45".to_string(),
                },
                Conversation {
                    name: "Charlie".to_string(),
                    avatar: "🧑".to_string(),
                    last_message: "周末见！".to_string(),
                    time: "15:20".to_string(),
                },
                Conversation {
                    name: "David".to_string(),
                    avatar: "👦".to_string(),
                    last_message: "收到，谢谢".to_string(),
                    time: "14:10".to_string(),
                },
            ],
            selected_conversation: Some(0),
            messages: vec![
                Message {
                    content: "嗨，最近怎么样？".to_string(),
                    is_self: false,
                    time: "18:20".to_string(),
                },
                Message {
                    content: "还不错，你呢？".to_string(),
                    is_self: true,
                    time: "18:22".to_string(),
                },
                Message {
                    content: "我也挺好的，晚上一起吃饭吗？".to_string(),
                    is_self: false,
                    time: "18:25".to_string(),
                },
                Message {
                    content: "好啊！去哪里吃？".to_string(),
                    is_self: true,
                    time: "18:28".to_string(),
                },
                Message {
                    content: "去那家新开的日料店？".to_string(),
                    is_self: false,
                    time: "18:30".to_string(),
                },
            ],
        }
    }

    // 选择会话
    fn select_conversation(&mut self, index: usize, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        self.selected_conversation = Some(index);
        cx.notify();
    }

    // 渲染左侧侧边栏
    fn render_sidebar(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w(px(280.))
            .h_full()
            .v_flex()
            .bg(rgb(0xf8f9fa))
            .border_r_1()
            .border_color(rgb(0xe9ecef))
            // 顶部标题栏
            .child(
                div()
                    .h(px(60.))
                    .p_4()
                    .border_b_1()
                    .border_color(rgb(0xe9ecef))
                    .flex()
                    .items_center()
                    .child(
                        div()
                            .text_xl()
                            .font_bold()
                            .text_color(rgb(0x212529))
                            .child("💬 消息")
                    )
            )
            // 会话列表
            .child(
                div()
                    .id("conversation-list")
                    .v_flex()
                    .flex_1()
                    .overflow_y_scrollbar()
                    .children(
                        self.conversations.iter().enumerate().map(|(index, conv)| {
                            let is_selected = self.selected_conversation == Some(index);
                            div()
                                .id(("conversation", index))
                                .h_flex()
                                .gap_3()
                                .p_3()
                                .items_center()
                                .cursor_pointer()
                                .bg(if is_selected { rgb(0xe7f3ff) } else { rgb(0xf8f9fa) })
                                .hover(|s| s.bg(rgb(0xf1f3f5)))
                                .on_click(cx.listener(move |this, event, window, cx| {
                                    this.select_conversation(index, event, window, cx);
                                }))
                                // 头像
                                .child(
                                    div()
                                        .size(px(48.))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .rounded(px(24.))
                                        .bg(rgb(0xdee2e6))
                                        .text_2xl()
                                        .child(conv.avatar.clone())
                                )
                                // 会话信息
                                .child(
                                    div()
                                        .v_flex()
                                        .gap_1()
                                        .flex_1()
                                        .child(
                                            div()
                                                .h_flex()
                                                .justify_between()
                                                .items_center()
                                                .child(
                                                    div()
                                                        .font_semibold()
                                                        .text_color(rgb(0x212529))
                                                        .child(conv.name.clone())
                                                )
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(rgb(0x868e96))
                                                        .child(conv.time.clone())
                                                )
                                        )
                                        .child(
                                            div()
                                                .text_sm()
                                                .text_color(rgb(0x868e96))
                                                .child(conv.last_message.clone())
                                        )
                                )
                        })
                    )
            )
    }

    // 渲染右侧聊天区域
    fn render_chat_area(&mut self, _cx: &mut Context<Self>) -> impl IntoElement {
        let selected_name = self.selected_conversation
            .and_then(|idx| self.conversations.get(idx))
            .map(|conv| conv.name.clone())
            .unwrap_or_else(|| "未选择".to_string());

        div()
            .flex_1()
            .h_full()
            .v_flex()
            .bg(rgb(0xffffff))
            // 顶部标题栏
            .child(
                div()
                    .h(px(60.))
                    .px_4()
                    .border_b_1()
                    .border_color(rgb(0xe9ecef))
                    .flex()
                    .items_center()
                    .child(
                        div()
                            .text_lg()
                            .font_semibold()
                            .text_color(rgb(0x212529))
                            .child(selected_name)
                    )
            )
            // 消息列表
            .child(
                div()
                    .id("message-list")
                    .flex_1()
                    .v_flex()
                    .gap_3()
                    .p_4()
                    .overflow_y_scrollbar()
                    .bg(rgb(0xf8f9fa))
                    .children(
                        self.messages.iter().map(|msg| {
                            div()
                                .h_flex()
                                .when(msg.is_self, |el| el.justify_end())
                                .child(
                                    div()
                                        .v_flex()
                                        .gap_1()
                                        .max_w(px(400.))
                                        .child(
                                            div()
                                                .p_3()
                                                .rounded(px(12.))
                                                .bg(if msg.is_self {
                                                    rgb(0x0084ff)
                                                } else {
                                                    rgb(0xffffff)
                                                })
                                                .text_color(if msg.is_self {
                                                    rgb(0xffffff)
                                                } else {
                                                    rgb(0x212529)
                                                })
                                                .border_1()
                                                .border_color(if msg.is_self {
                                                    rgb(0x0084ff)
                                                } else {
                                                    rgb(0xdee2e6)
                                                })
                                                .child(msg.content.clone())
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(rgb(0x868e96))
                                                .when(msg.is_self, |el| el.text_right())
                                                .child(msg.time.clone())
                                        )
                                )
                        })
                    )
            )
            // 底部输入区域
            .child(
                div()
                    .h(px(80.))
                    .p_4()
                    .border_t_1()
                    .border_color(rgb(0xe9ecef))
                    .h_flex()
                    .gap_2()
                    .items_center()
                    .child(
                        div()
                            .flex_1()
                            .h(px(48.))
                            .px_3()
                            .rounded(px(24.))
                            .border_1()
                            .border_color(rgb(0xdee2e6))
                            .flex()
                            .items_center()
                            .text_color(rgb(0x868e96))
                            .child("输入消息...")
                    )
                    .child(
                        Button::new("send-btn")
                            .primary()
                            .label("发送")
                    )
            )
    }
}

impl Render for ChatApp {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .h_flex()
            .size_full()
            .child(self.render_sidebar(cx))
            .child(self.render_chat_area(cx))
    }
}

fn main() {
    let app = Application::new();

    app.run(move |cx| {
        // This must be called before using any GPUI Component features.
        gpui_component::init(cx);

        init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|_| ChatApp::new());
                // This first level on the window, should be a Root.
                cx.new(|cx| Root::new(view, window, cx))
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}

fn init(cx: &mut App) {
    let theme_name = SharedString::from("Ayu Light");
    // Load and watch themes from ./themes directory
    if let Err(err) = ThemeRegistry::watch_dir(PathBuf::from("./themes"), cx, move |cx| {
        if let Some(theme) = ThemeRegistry::global(cx)
            .themes()
            .get(&theme_name)
            .cloned()
        {
            Theme::global_mut(cx).apply_config(&theme);
        }
    }) {
        panic!("Failed to watch themes directory: {}", err);
    }
}