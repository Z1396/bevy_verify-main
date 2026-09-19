//! 敌人插件 —— 敌人玩法逻辑的挂载点（当前为占位实现）。

use bevy::prelude::*;

/// 敌人插件：预留的敌人系统注册入口。
///
/// 组件定义见 `components::Enemy` / `components::Health`；
/// 后续可在此注册敌人 AI、受击、死亡等系统。
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    /// 插件装配入口：当前无系统需要注册，保持空实现。
    fn build(&self, _app: &mut App) {
        // Enemy 相关系统在此注册
        // 组件已在 components.rs 中定义
    }
}
