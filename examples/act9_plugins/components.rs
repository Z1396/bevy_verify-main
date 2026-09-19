//! 游戏组件定义 —— 纯数据结构，不包含任何逻辑。
//!
//! 组件挂在实体上，由各插件注册的系统通过 Query 按需访问；
//! 数据（本模块）与行为（systems.rs）分离是 ECS 的核心原则。

use bevy::prelude::*;

/// 玩家标记组件：零大小类型，仅用于 Query 筛选。
#[derive(Component)]
pub struct Player;

/// 敌人标记组件：零大小类型，仅用于 Query 筛选。
#[derive(Component)]
pub struct Enemy;

/// 生命值组件。
///
/// # 字段
/// - `0`（元组字段）：剩余生命点数，降到 0 判定死亡。
#[derive(Component)]
#[allow(dead_code)] // 演示字段：本例只演示组件定义，无系统读取
pub struct Health(pub u32);
