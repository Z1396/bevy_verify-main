//! 玩家插件 —— 聚合玩家相关的系统注册。

use bevy::prelude::*;

use crate::systems::move_player;

/// 玩家插件：负责注册玩家移动系统。
///
/// 组件定义见 `components::Player`，系统实现见 `systems::move_player`。
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    /// 插件装配入口：向应用注册本插件拥有的系统。
    ///
    /// # 参数
    /// - `app`：Bevy 应用实例，通过它把系统加入调度。
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_player);
    }
}
