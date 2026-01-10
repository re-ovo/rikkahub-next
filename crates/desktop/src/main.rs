//! RikkaHub 桌面客户端

mod app;
mod client;

use anyhow::Result;
use gpui::*;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn main() -> Result<()> {
    // 初始化日志
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    tracing::info!("启动 RikkaHub Desktop");

    // 启动 GPUI 应用
    Application::new().run(|cx| {
        // 初始化 gpui-component
        gpui_component::init(cx);

        // 打开主窗口
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(1200.0), px(800.0)),
                    cx,
                ))),
                ..Default::default()
            },
            |_window, cx| cx.new(|_cx| app::RikkaApp::new()),
        )
        .unwrap();
    });

    Ok(())
}
