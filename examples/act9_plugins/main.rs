//! Act 9: Plugin 插件 — impl Plugin / PluginGroup / add_plugins / 模块化。
//!
//! 演示 Bevy 的代码组织单元：
//! - `Plugin`：把相关的资源、组件、系统打包成可复用模块；
//! - `PluginGroup`：把多个插件聚合为一组，一次 `add_plugins` 全部注册；
//! - 与 Act 10 的区别：本例侧重插件化结构，Act 10 侧重完整玩法闭环。
//!
//! 模块划分：
//! - `components`：组件（纯数据）；
//! - `resources`：资源（全局状态）；
//! - `systems`：系统（业务逻辑函数）；
//! - `plugins`：插件与插件组（装配与注册）。
//!
//! 运行方式：`cargo run --example act9_plugins`

mod components;
mod plugins;
mod resources;
mod systems;

use bevy::prelude::*;
use plugins::GamePlugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // 一行引入全部游戏插件（PluginGroup），等价于逐个注册三个插件
        .add_plugins(GamePlugins)
        .run();
}
