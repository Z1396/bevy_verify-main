//! Act 10: 实战 Mini 游戏 — 玩家移动 + 射击 + 碰撞 + 计分。
//!
//! 综合运用 Act 1-9 的概念实现完整玩法闭环：
//! - 移动：WASD 控制（Act 6 输入 + Act 5 查询）；
//! - 射击：空格发射子弹，`BulletFired` 消息解耦"输入触发"与"实体生成"（Act 8）；
//! - 碰撞：子弹 × 敌人距离检测，命中后双方销毁并加分（Act 7 资源）；
//! - UI：`ScoreDisplay` 实时显示屏幕左上角的分数。
//!
//! 模块划分：
//! - `components`：玩家 / 敌人 / 子弹组件；
//! - `messages`：子弹发射消息；
//! - `resources`：分数资源；
//! - `systems`：全部玩法系统；
//! - `score_ui`：分数 UI 的创建与刷新。
//!
//! 运行方式：`cargo run --example act10_game`

mod components;
mod messages;
mod resources;
mod score_ui;
mod systems;

use bevy::prelude::*;
use messages::BulletFired;
use resources::Score;
use score_ui::{setup_score_display, update_score_display};
use systems::{
    check_collision, cleanup_bullets, move_bullet, move_player, setup, shoot, spawn_bullet,
};

/// 程序入口：装配应用并进入主循环。
///
/// Update 阶段的系统顺序有讲究：
/// 1. `move_player` → 读取输入，移动玩家；
/// 2. `shoot` → 空格按下时发出 `BulletFired` 消息（带玩家当前位置）；
/// 3. `spawn_bullet` → 消费消息，生成子弹实体；
/// 4. `move_bullet` → 移动所有子弹；
/// 5. `check_collision` → 命中检测，销毁实体并计分；
/// 6. `cleanup_bullets` → 清理飞出屏幕的子弹；
/// 7. `update_score_display` → 同步分数到 UI。
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .add_message::<BulletFired>()
        .add_systems(Startup, (setup, setup_score_display))
        .add_systems(Update, (
            move_player,
            shoot,
            spawn_bullet,
            move_bullet,
            check_collision,
            cleanup_bullets,
            update_score_display,
        ))
        .run();
}
