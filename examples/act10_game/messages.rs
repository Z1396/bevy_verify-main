//! 游戏消息定义 —— 系统间解耦通信的事件载体。

use bevy::prelude::*;

/// 子弹发射消息。
///
/// `shoot` 系统在空格按下时发出，`spawn_bullet` 系统消费后在对应
/// 位置生成子弹实体，实现"输入触发"与"实体生成"的解耦：
/// 射击逻辑变更无需改动子弹生成逻辑，反之亦然。
///
/// # 字段
/// - `position`：发射起点（玩家当前坐标，世界空间）。
#[derive(Message)]
pub struct BulletFired {
    pub position: Vec2,
}
