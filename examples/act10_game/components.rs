//! 游戏实体组件定义 —— 玩家、敌人与子弹的纯数据。
//!
//! 组件挂在实体上，由 systems.rs 中的系统通过 Query 按需访问。

use bevy::prelude::*;

/// 玩家标记组件：零大小类型，仅用于 Query 筛选。
#[derive(Component)]
pub struct Player;

/// 敌人标记组件：零大小类型，仅用于 Query 筛选。
#[derive(Component)]
pub struct Enemy;

/// 子弹组件，携带弹道数据。
///
/// # 字段
/// - `speed`：飞行速度，单位 像素/秒，沿 +Y 方向（向上）飞行。
#[derive(Component)]
pub struct Bullet {
    pub speed: f32,
}
