//! Act 2: 第一个精灵 — Camera2d + Sprite::from_color。
//!
//! 演示 2D 渲染的最小单位：
//! - `Camera2d`：2D 相机，没有它渲染管线不会输出任何画面；
//! - `Sprite`：纯色方块精灵，由颜色 + 尺寸构成。
//!
//! 运行方式：`cargo run --example act2_first_sprite`

use bevy::prelude::*;

/// 场景初始化系统：创建相机与精灵实体。
///
/// # 参数
/// - `commands`：命令队列，生成的实体在帧末统一应用。
fn setup(mut commands: Commands) {
    // 必须有一个 Camera2d 才能看到画面
    commands.spawn(Camera2d);

    // 创建一个纯色方块精灵（100×100 像素）
    // - 颜色：sRGB 空间 (0.2, 0.6, 1.0)，浅蓝色
    // - 尺寸：以像素为单位，锚点为几何中心（未指定 Transform 时位于原点）
    commands.spawn(Sprite::from_color(
        Color::srgb(0.2, 0.6, 1.0),
        Vec2::new(100.0, 100.0),
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Startup Schedule：仅在应用启动时执行一次
        .add_systems(Startup, setup)
        .run();
}
