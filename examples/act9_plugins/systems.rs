//! 游戏系统集合 —— 所有业务逻辑函数。
//!
//! 系统按归属分组：通用初始化（跨插件共享）、玩家、计分。
//! 每个插件只注册与自己相关的系统，保持模块间解耦。

use bevy::prelude::*;

use crate::components::{Enemy, Health, Player};
use crate::resources::Score;

// ── 通用 Setup（跨插件共享） ──────────────────────────

/// 场景初始化：创建相机、玩家与敌人实体。
///
/// 目前由 `ScorePlugin` 代为注册（它承担了基础场景搭建的职责）；
/// 后续若插件增多，可拆分为独立的 `SetupPlugin`。
///
/// # 参数
/// - `commands`：命令队列，生成的实体在帧末统一应用。
pub fn setup_common(mut commands: Commands) {
    // 2D 相机
    commands.spawn(Camera2d);
    // 玩家：蓝色方块，位于原点
    commands.spawn((
        Sprite::from_color(Color::srgb(0.2, 0.6, 1.0), Vec2::new(50.0, 50.0)),
        Player,
    ));
    // 敌人：红色方块，携带 3 点生命值
    commands.spawn((
        Sprite::from_color(Color::srgb(1.0, 0.3, 0.3), Vec2::new(40.0, 40.0)),
        Transform::from_xyz(100.0, 200.0, 0.0),
        Enemy,
        Health(3),
    ));
}

// ── Player 系统 ──────────────────────────────────────

/// 玩家移动系统：WASD 四方向移动，方向可叠加。
///
/// # 参数
/// - `input`：只读键盘输入；
/// - `time`：帧间隔，乘出本帧位移，保证帧率无关；
/// - `query`：可变查询所有玩家的 `Transform`。
pub fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let speed = 300.0 * time.delta_secs();
    for mut tf in &mut query {
        if input.pressed(KeyCode::KeyW) {
            tf.translation.y += speed;
        }
        if input.pressed(KeyCode::KeyS) {
            tf.translation.y -= speed;
        }
        if input.pressed(KeyCode::KeyA) {
            tf.translation.x -= speed;
        }
        if input.pressed(KeyCode::KeyD) {
            tf.translation.x += speed;
        }
    }
}

// ── Score 系统 ───────────────────────────────────────

/// 分数打印系统：仅在 `Score` 发生变化的那一帧输出，避免每帧刷屏。
///
/// `is_changed()` 依赖 Bevy 的变更检测（change detection）：
/// 只要上一帧以来有系统通过 `ResMut` 写入过该资源，即返回 true。
pub fn show_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.total);
    }
}
