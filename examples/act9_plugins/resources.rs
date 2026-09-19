//! 游戏资源定义 —— 全局唯一的共享状态。
//!
//! 资源与实体无关，整个 World 中只有一个实例，
//! 系统通过 `Res<T>` / `ResMut<T>` 访问，并在插件 build 时初始化。

use bevy::prelude::*;

/// 全局计分板资源。
///
/// # 字段
/// - `total`：累计得分，由 `ScorePlugin::build` 中 `init_resource` 以 0 初始化。
#[derive(Resource, Default)]
pub struct Score {
    pub total: u32,
}
