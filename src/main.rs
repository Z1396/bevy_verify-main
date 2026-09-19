//! Bevy 编程入门 — 代码合集（各章节独立示例见 examples/）
//!
//! 本文件将各章核心概念整合为一个可直接运行的演示程序：
//! - 组件（Component）：`Player` / `Health`，挂在实体上的数据片段；
//! - 资源（Resource）：`Score`，全局唯一的共享状态；
//! - 消息（Message）：`PlayerDied`，系统间解耦的通信方式；
//! - 系统（System）：普通函数，通过参数类型自动声明数据依赖。
//!
//! 运行方式：`cargo run`

use bevy::prelude::*;

// ── 组件 ────────────────────────────────────────────────────────────────

/// 玩家标记组件。
///
/// 零大小类型（ZST），不携带数据，仅用于在 Query 中筛选玩家实体，
/// 例如 `Query<&mut Transform, With<Player>>`。
#[derive(Component, Debug)]
struct Player;

/// 玩家生命值组件。
///
/// # 字段
/// - `current`：当前生命值。初始 100.0，降至 0.0 及以下判定死亡。
#[derive(Component, Debug)]
struct Health {
    current: f32,
}

// ── 资源 ────────────────────────────────────────────────────────────────

/// 全局计分板资源。
///
/// # 字段
/// - `total`：累计得分（非负整数），由 `init_resource` 以默认值 0 初始化。
#[derive(Resource, Default, Debug)]
struct Score {
    total: u32,
}

// ── 消息 ────────────────────────────────────────────────────────────────

/// 玩家死亡消息。
///
/// 由 `death_system` 在检测到生命值耗尽时发出，
/// 由 `score_system` 消费并把分数计入 `Score`。
///
/// # 字段
/// - `score`：本次死亡结算的分数（示例固定为 100）。
#[derive(Message, Debug)]
struct PlayerDied {
    score: u32,
}

// ── 系统 ────────────────────────────────────────────────────────────────

/// 初始化场景：创建 2D 相机与玩家实体。
///
/// # 参数
/// - `commands`：命令队列，生成的实体在帧末统一应用，
///   避免与并行系统的借用检查冲突。
/*这里用了一个元组（tuple）作为 Bundle。Bevy 允许把任意多个 Component 直接写成元组，编译器会自动把它当成一个 Bundle，不需要你手动 #[derive(Bundle)]。

元组里有三个组件：

Sprite::from_color(...)：精灵外观，决定画什么颜色、多大。

Player：你自己定义的标记组件（Marker Component），通常是个空结构体 struct Player;。它的作用是“给这个实体打个标签”，方便后续系统用 Query<&Player> 筛选出玩家实体。

Health { current: 100.0 }：你自己定义的数据组件，存血量。 */
fn setup(mut commands: Commands) {
    // 2D 渲染必须有相机，否则画布为空
    commands.spawn(Camera2d::default());
    // 元组形式同时挂载多个组件（Bundle）：精灵外观 + 玩家标记 + 生命值
    commands.spawn((
        Sprite::from_color(Color::srgb(0.8, 0.6, 1.0), Vec2::new(50.0, 50.0)),
        Player,
        Health { current: 100.0 },
    ));
}

/// 玩家移动系统：根据 WASD 按键状态修改 `Transform`。
///
/// # 参数
/// - `input`：只读键盘输入资源；
/// - `time`：帧时间资源，`delta_secs()` 使移动速度与帧率无关；
/// - `query`：可变查询所有携带 `Player` 组件的 `Transform`。
fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    //一句话：找到所有带 Player 标记的实体，拿到它们的 Transform，可以修改位置。
    mut query: Query<&mut Transform, With<Player>>,
) {
    // 本帧位移 = 速度(300 像素/秒) × 帧间隔(秒)
    let speed = 300.0 * time.delta_secs();
    for mut tf in &mut query {
        if input.pressed(KeyCode::KeyW) { tf.translation.y += speed; }
        if input.pressed(KeyCode::KeyS) { tf.translation.y -= speed; }
        if input.pressed(KeyCode::KeyA) { tf.translation.x -= speed; }
        if input.pressed(KeyCode::KeyD) { tf.translation.x += speed; }
    }
}

/// 计分系统（资源写入演示）：每帧固定加 10 分。
fn add_score(mut score: ResMut<Score>) {
    score.total += 10;
}

/// 计分系统（资源读取演示）：每帧打印当前总分。
fn show_score(score: Res<Score>) {
    println!("Score: {}", score.total);
}

/// 死亡检测系统：生命值耗尽时广播 `PlayerDied` 消息。
///
/// 写入方只负责发出消息，不关心谁来消费 —— 消息机制的系统解耦价值。
///
/// # 参数
/// - `writer`：消息写入端，将消息入队等待读取方消费；
/// - `query`：读取玩家 `Health` 组件。
fn death_system(
    mut writer: MessageWriter<PlayerDied>,
    query: Query<&Health, With<Player>>,
) {
    for health in &query {
        if health.current <= 0.0 {
            writer.write(PlayerDied { score: 100 });
        }
    }
}

/// 分数结算系统：消费 `PlayerDied` 消息并累计得分。
///
/// # 参数
/// - `reader`：消息读取端，内置游标保证同一消息不会被重复消费；
/// - `score`：可写访问全局 `Score` 资源。
fn score_system(mut reader: MessageReader<PlayerDied>, mut score: ResMut<Score>) {
    for death in reader.read() {
        score.total += death.score;
    }
}

// ── 入口 ────────────────────────────────────────────────────────────────

/// 程序入口：装配应用并进入主循环。
///
/// 装配顺序：
/// 1. `DefaultPlugins`：窗口 / 渲染 / 输入等基础能力；
/// 2. `init_resource`：注册全局分数资源（`Score` 需实现 `Default`）；
/// 3. `add_message`：注册消息类型（使用前必须注册）；
/// 4. `add_systems`：`Startup` 仅启动时执行一次，`Update` 每帧执行。
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .add_message::<PlayerDied>()
        .add_systems(Startup, setup)
        .add_systems(Update, (move_player, add_score, show_score, death_system, score_system))
        .run();
}
