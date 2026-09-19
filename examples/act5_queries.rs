//! Act 5: Query 查询 — Query<&T> / Query<&mut T> / With<T> 过滤器。
//!
//! 演示系统访问实体数据的核心方式 Query：
//! - `Query<&mut Transform, With<Player>>`：可变访问玩家实体的变换；
//! - `With<T>`：只保留同时携带组件 T 的实体，仅作过滤、不取其数据；
//! - 调度器依据 Query 的读写需求做借用检查，冲突的系统不会并行。
//!
//! 运行方式：`cargo run --example act5_queries`

use bevy::prelude::*;

/// 玩家标记组件。
#[derive(Component)]
struct Player;

/// 敌人标记组件。
#[derive(Component)]
struct Enemy;

/// 玩家移动系统：每帧沿 +X 方向匀速移动所有玩家实体。
///
/// # 参数
/// - `query`：可变查询携带 `Player` 的 `Transform`；
/// - `time`：帧间隔，保证帧率无关的恒定速度。
fn move_player(mut query: Query<&mut Transform, With<Player>>, time: Res<Time>) {
    let speed = 200.0 * time.delta_secs();
    for mut tf in &mut query {
        tf.translation.x += speed;
    }
}

/// 敌人位置打印系统：只读遍历所有敌人实体。
///
/// # 参数
/// - `query`：只读查询携带 `Enemy` 的 `Transform`。
fn print_enemies(query: Query<&Transform, With<Enemy>>) {
    for tf in &query {
        println!("Enemy at: ({:.1}, {:.1})", tf.translation.x, tf.translation.y);
    }
}

/// 场景初始化：生成玩家与敌人各一个。
///
/// # 参数
/// - `commands`：命令队列，生成的实体在帧末统一应用。
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Player 实体：蓝色方块，初始位于 X = -200
    commands.spawn((
        Sprite::from_color(Color::srgb(0.2, 0.6, 1.0), Vec2::new(50.0, 50.0)),
        Transform::from_xyz(-200.0, 0.0, 0.0),
        Player,
    ));

    // Enemy 实体：红色方块，初始位于 X = 200
    commands.spawn((
        Sprite::from_color(Color::srgb(1.0, 0.3, 0.3), Vec2::new(40.0, 40.0)),
        Transform::from_xyz(200.0, 0.0, 0.0),
        Enemy,
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (move_player, print_enemies))
        .run();
}
