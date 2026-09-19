//! Act 3: 组件 Component — #[derive(Component)] + spawn 附加。
//!
//! 演示组件的定义与挂载：
//! - 任意 Rust 结构体派生 `Component` 即成为组件；
//! - `spawn((A, B, C, ...))` 以元组 Bundle 形式一次性挂载多个组件；
//! - 组件是纯数据，逻辑一律放在系统中（ECS 数据与行为分离）。
//!
//! 运行方式：`cargo run --example act3_components`

use bevy::prelude::*;

/// 玩家组件：携带移动速度。
///
/// # 字段
/// - `speed`：移动速度，单位 像素/秒。
#[derive(Component, Debug)]
#[allow(dead_code)] // 演示字段：本例只演示组件挂载，无系统读取
struct Player {
    speed: f32,
}

/// 生命值组件。
///
/// # 字段
/// - `current`：当前生命值，初始 100.0。
#[derive(Component, Debug)]
#[allow(dead_code)] // 演示字段：本例只演示组件挂载，无系统读取
struct Health {
    current: f32,
}

/// 场景初始化：生成一个携带四个组件的玩家实体。
///
/// # 参数
/// - `commands`：命令队列，生成的实体在帧末统一应用。
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // spawn 时附加多个组件（Bundle 元组）
    // 同一实体同时拥有：渲染外观、空间位置、玩家数据、生命值
    commands.spawn((
        Sprite::from_color(Color::srgb(0.2, 0.6, 1.0), Vec2::new(50.0, 50.0)),
        Transform::from_xyz(100.0, 0.0, 0.0),
        Player { speed: 300.0 },
        Health { current: 100.0 },
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}
