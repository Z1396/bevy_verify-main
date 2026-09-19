//! 游戏全局资源定义。

use bevy::prelude::*;

/// 全局计分板资源。
///
/// # 字段
/// - `total`：累计得分；击毁一个敌人 +100。
///   由 `main` 中 `init_resource` 以默认值 0 初始化。
#[derive(Resource, Default)]
pub struct Score {
    pub total: u32,
}
