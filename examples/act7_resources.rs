//! Act 7: Resource 资源 — Res<T> / ResMut<T> / init_resource / insert_resource。
//!
//! 演示全局状态的读写：
//! - 资源是整个 World 中唯一的实例，与具体实体无关；
//! - `Res<T>` 只读访问、`ResMut<T>` 可写访问，由调度器做借用检查；
//! - `init_resource` 用 `Default` 初始化，`insert_resource` 直接给定值。
//!
//! 运行方式：`cargo run --example act7_resources`

use bevy::prelude::*;

/// 计分板资源（实现 `Default` 以支持 `init_resource`）。
///
/// # 字段
/// - `total`：累计得分，每次加分 +100；
/// - `multiplier`：分数倍率，每次加分后递增 0.1（仅作演示）。
#[derive(Resource, Default, Debug)]
struct Score {
    total: u32,
    multiplier: f32,
}

/// 只读访问演示：打印当前分数与倍率。
///
/// # 参数
/// - `score`：`Res<Score>` 只读句柄，同帧可有多个只读系统并行。
fn show_score(score: Res<Score>) {
    println!("Score: {} (x{:.1})", score.total, score.multiplier);
}

/// 可写访问演示：每次执行加分并提升倍率。
///
/// # 参数
/// - `score`：`ResMut<Score>` 可写句柄，与其他访问同一资源的系统互斥。
fn add_score(mut score: ResMut<Score>) {
    score.total += 100;
    score.multiplier += 0.1;
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // 4. 初始化资源（方式一：默认值，要求 T: Default）
        .init_resource::<Score>()
        // 方式二：自定义初始值
        // .insert_resource(Score { total: 0, multiplier: 1.0 })
        .add_systems(Update, (add_score, show_score))
        .run();
}
