//! 游戏插件模块 —— 定义 `GamePlugins` 插件组并聚合各功能插件。
//!
//! 插件划分原则：每个玩法域一个插件，各自注册自己的系统与资源；
//! `GamePlugins` 作为统一入口，调用方一行代码即可完成装配。

pub mod enemy;
pub mod player;
pub mod score;

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;

/// 游戏插件组 —— 打包一组插件（`DefaultPlugins` 同样是 `PluginGroup`）。
///
/// 聚合内容：
/// 1. `PlayerPlugin`：注册玩家移动系统；
/// 2. `EnemyPlugin`：敌人相关系统（当前为占位实现）；
/// 3. `ScorePlugin`：初始化分数资源、搭建场景、注册计分系统。
pub struct GamePlugins;

impl PluginGroup for GamePlugins {
    /// 按序装配全部游戏插件。
    ///
    /// `PluginGroupBuilder` 还支持 `insert_before` / `insert_after` /
    /// `disable` 等操作，可细粒度调整组内插件的顺序与开关。
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(PlayerPlugin)
            .add(EnemyPlugin)
            .add(ScorePlugin)
    }
}
