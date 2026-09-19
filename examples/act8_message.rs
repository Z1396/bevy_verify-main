//! Act 8: Message 消息系统 — MessageWriter / MessageReader / EntityEvent。
//!
//! 演示两种事件通信机制：
//! - 全局消息（Part 1-4）：`MessageWriter<T>` 发送、`MessageReader<T>` 消费，
//!   适合系统间解耦广播；
//! - 实体事件（Part 5）：`EntityEvent` + `observe`，事件只投递给挂在
//!   目标实体上的观察者，适合精确的实体级响应。
//!
//! 运行方式：`cargo run --example act8_message`

use bevy::prelude::*;

// ═══════════════════════════════════════════════════════════════════════
// Part 1-4: 全局 Message —— MessageWriter / MessageReader
// ═══════════════════════════════════════════════════════════════════════

/// 玩家死亡消息（消息载荷）。
///
/// # 字段
/// - `score`：死亡结算分数，由消费方累加。
#[derive(Message, Debug)]
struct PlayerDied {
    score: u32,
}

/// 分数资源：消息携带数据的接收方。
///
/// # 字段
/// - `total`：累计得分。
#[derive(Resource, Default)]
struct Score {
    total: u32,
}

/// 发送方：每帧广播一条 `PlayerDied` 消息。
///
/// 写入方只负责入队消息，不关心谁来消费 —— 这就是消息机制的解耦价值。
fn death_system(mut writer: MessageWriter<PlayerDied>) {
    writer.write(PlayerDied { score: 100 });
    println!("Sent: PlayerDied {{ score: 100 }}");
}

/// 接收方：读取本帧内新到达的消息并累计分数。
///
/// # 参数
/// - `reader`：读取端内置游标，同一消息不会被重复消费，
///   系统暂停多帧后恢复也能按序补读；
/// - `score`：可写访问分数资源。
fn score_system(mut reader: MessageReader<PlayerDied>, mut score: ResMut<Score>) {
    for death in reader.read() {
        score.total += death.score;
        println!("Received: score += {} → total = {}", death.score, score.total);
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Part 5: EntityEvent —— 绑定到特定实体的 Observer
// ═══════════════════════════════════════════════════════════════════════

/// 实体级死亡事件：只投递给目标实体上注册的观察者。
///
/// 派生 `EntityEvent` 要求结构体携带 `entity: Entity` 字段，
/// 用于指明事件的目标实体。
///
/// # 字段
/// - `entity`：事件目标实体；
/// - `points`：击杀奖励分数。
#[derive(EntityEvent, Debug)]
struct EntityDied {
    entity: Entity,
    points: u32,
}

/// 生成一个带观察者的敌人实体。
///
/// `observe` 闭包作为 Observer 挂在该实体上：当实体收到 `EntityDied`
/// 事件时触发（事件驱动，而非每帧轮询）。
fn spawn_entity(mut commands: Commands) {
    commands
        .spawn((
            Name::new("Enemy"),
            Transform::default(),
        ))
        .observe(|on: On<EntityDied>| {
            println!(
                "Entity died! +{} points → entity: {:?}",
                on.event().points,
                on.observer(),
            );
        });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        // 4. 注册消息类型（必须先注册才能使用）
        .add_message::<PlayerDied>()
        // 系统顺序：Writer 在 Reader 之前（同一帧内先发后收）
        .add_systems(Startup, spawn_entity)
        .add_systems(Update, (death_system, score_system))
        .run();
}
