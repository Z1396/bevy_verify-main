//! 计分插件 —— 初始化分数资源并注册计分相关系统。

use bevy::prelude::*;

use crate::resources::Score;
use crate::systems::{setup_common, show_score};

/// 计分插件：负责资源初始化、场景搭建与分数展示。
pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    /// 插件装配入口：
    ///
    /// 1. `init_resource::<Score>()`：以默认值注册全局分数资源；
    /// 2. `setup_common`（Startup）：创建相机与玩家/敌人实体；
    /// 3. `show_score`（Update）：分数变化时打印。
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_systems(Startup, setup_common)
            .add_systems(Update, show_score);
    }
}
