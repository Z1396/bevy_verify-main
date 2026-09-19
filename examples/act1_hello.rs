//! Act 1: Hello Bevy — 三行代码启动 Bevy 应用。
//!
//! 演示 Bevy 应用的最小骨架：
//! - `App::new` 创建应用实例；
//! - `add_plugins(DefaultPlugins)` 装配窗口、渲染、输入等内置插件；
//! - `App::run` 进入事件驱动主循环，直到窗口关闭。
//!
//! 运行方式：`cargo run --example act1_hello`

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // 内置插件组：窗口/渲染/输入/资源加载等
        .run(); // 阻塞式主循环，每帧调度所有 Schedule
}
