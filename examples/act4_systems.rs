//! Act 4: 系统 System — fn 函数 + add_systems 注册。
//!
//! 演示系统的定义与注册：
//! - 系统就是普通 Rust 函数，通过参数类型（`Res` / `Query` 等）声明数据依赖；
//! - `Update` Schedule 每帧执行一次；
//! - 数据依赖互不冲突的系统会被调度器自动并行执行。
//!
//! 运行方式：`cargo run --example act4_systems`

use bevy::prelude::*;

/// 演示系统 A：无参数，仅打印一行日志。
fn hello_system() {
    println!("Hello from Bevy system!");
}

/// 演示系统 B：只读访问全局时间资源。
///
/// # 参数
/// - `time`：`Res<Time>` 只读句柄；`delta_secs()` 返回上一帧到
///   当前帧的间隔（秒），可用于实现帧率无关的动画。
fn count_system(time: Res<Time>) {
    // 每帧打印一次（运行 -qh 模式会更明显）
    println!("Delta time: {:.4}s", time.delta_secs());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // 多个系统注册到同一个 Schedule
        // 元组形式批量注册，二者只读不写、无冲突，可并行执行
        .add_systems(Update, (hello_system, count_system))
        .run();
}
